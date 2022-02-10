use std::io;
use std::io::Write;
use std::thread;
use crate::domain::model::command::Command;
use crate::infra::os_manager;
use crate::utils::parser_helper;
use crate::utils::file_helper;

#[derive(Clone)]
pub struct BashManager {
  pub username: String,
  pub history: Vec<String>,
  pub paths: Vec<String>,
  pub aliases: Vec<(String, String)>
}

impl BashManager {
  pub fn enqueue_command(&mut self, command: String){
    if self.history.len() >= 10 {
      self.history.remove(0);
    }
    self.history.push(command)
  }

  pub fn show_path(&self) {
    print!("Unb - {} - {} ", self.username, os_manager::get_current_directory());
    io::stdout().flush().map_err(|err| println!("{:?}", err)).ok();
  }

  pub fn read_command(&self) -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|err| println!("{:?}", err)).ok();
    input
  }

  pub fn decode_command(&self, command: &mut Command) -> Command {
    for alias in &self.aliases {
      if alias.1 == command.command_name {
        let decoded_command = Command {command_name: alias.0.clone(), args: command.args.clone()};
        return decoded_command
      }
    }
    let decoded_command = Command {command_name: command.command_name.clone(), args: command.args.clone()};
    return decoded_command
  }

  pub fn execute(&self, pipe_sections: &Vec<Command>){
    let mut i = 0;
    let last_index = pipe_sections.len() - 1;
    let mut output_added = false;
    let mut home = os_manager::get_home_directory();
    home.push_str("/.unbsh_temp");
    for section in pipe_sections.clone().iter_mut() {
      if output_added {
        section.args.push(home.clone());
        output_added = false;
      }
      let decoded_section = self.decode_command(section);
      match decoded_section.command_name.as_str() {
        "cd" => os_manager::cd(decoded_section.args.clone()),
        "ver" => os_manager::ver(decoded_section.args.clone()),
        "history" => os_manager::history(decoded_section.args.clone(), (*self).clone()),
        _ => {
          if decoded_section.args.iter().any(|i| i=="<" || i==">" || i==">>") {
            print!("{}", os_manager::redir(decoded_section.clone(), (*self).clone()));
            return
          }
          let output = os_manager::execute_command(decoded_section.clone(), (*self).clone());
          file_helper::delete_file(home.clone());
          if i == last_index {
            print!("{}", output)
          } else{
            file_helper::create_write_file(home.clone(), output);
            output_added = true;
          }
        }
      }
      i += 1;
    }
  }

  pub fn parse_command(&mut self, command: String) -> Vec<Command> {
    let command: Vec<Command> = parser_helper::parse_commandline(command);
    command
  }

  pub fn execute_async(&mut self, pipe_sections: &Vec<Command>, input_command: &String, last_pipe_index: usize, last_arg_index: usize) {
    let clone_self = self.clone();
    let mut clone_pipe_sections = pipe_sections.clone();
    let clone_input_command = input_command.clone();
    println!("Processo em background [{}] foi iniciado", 1);
    clone_pipe_sections[last_pipe_index].args.remove(last_arg_index);
    thread::spawn(move || {
      clone_self.execute(&clone_pipe_sections);
      print!("Processo em background [{}] executado {}", 1, clone_input_command);
      clone_self.show_path()
    });
  }

  pub fn execute_batch(&mut self, pipe_sections: &Vec<Command>) {
    if pipe_sections.len() > 1 {
      println!("Mais de uma pipe_section");
    } else {
      if pipe_sections[0].args.len() > 0 {
        println!("Não passar argumento ao executar o  programa");
      } else {
        let command = pipe_sections[0].command_name.replace("./", "");
        let lines = parser_helper::parse_batch_program(command);
        for line in lines {
          let pipe_sections: Vec<Command> = parser_helper::parse_commandline(line.clone());
          self.sync_async_execute(&pipe_sections, &line);
        }
      }
    }
  }

  pub fn sync_async_execute(&mut self, pipe_sections: &Vec<Command>, input_command: &String) {
    let last_pipe_index = pipe_sections.len()-1;
    if pipe_sections[last_pipe_index].args.len() > 0 && pipe_sections[last_pipe_index].args[pipe_sections[last_pipe_index].args.len()-1] == "&" {
      let last_arg_index = pipe_sections[last_pipe_index].args.len() - 1;
      //tirar os argumentos de index e tirar o & daqui
      self.execute_async(&pipe_sections, &input_command, last_pipe_index, last_arg_index)
    } else {
      self.execute(&pipe_sections);
    }
  }
   
  pub fn run(&mut self) {
    self.show_path();
    let command = self.read_command();
    if command.len() == 1 {
      self.run();
      return
    }
    
    let pipe_sections: Vec<Command> = self.parse_command(command.clone());
    if pipe_sections[0].command_name.eq("exit") {
      return
    }

    if pipe_sections[0].command_name.contains("./"){
      self.execute_batch(&pipe_sections);
    } else{
      self.sync_async_execute(&pipe_sections, &command)
    }
    
    self.enqueue_command(command);
    self.run();
  }

}
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
  pub aliases: Vec<(String, String)>,
  pub process_id: i64
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
    let mut output : String = "".to_string();
    for section in pipe_sections.clone().iter_mut() {
      let decoded_section = self.decode_command(section);
      match decoded_section.command_name.as_str() {
        "cd" => output = os_manager::cd(decoded_section.args.clone()),
        "ver" => output = os_manager::ver(decoded_section.args.clone()),
        "history" => output = os_manager::history(decoded_section.args.clone(), (*self).clone()),
        _ => {
          if decoded_section.args.iter().any(|i| i=="<" || i==">" || i==">>") {
            output = os_manager::redir(decoded_section.clone(), (*self).clone());
          } else {
            output = os_manager::execute_command(decoded_section.clone(), (*self).clone(), output.clone(), pipe_sections.len() == 1);
          }
        }
      }
    }
    print!("{}", output);
  }

  pub fn parse_command(&mut self, command: String) -> Vec<Command> {
    let command: Vec<Command> = parser_helper::parse_commandline(command);
    command
  }

  pub fn execute_async(&mut self, pipe_sections: &Vec<Command>, input_command: &String, last_pipe_index: usize, last_arg_index: usize) {
    let clone_self = self.clone();
    let mut clone_pipe_sections = pipe_sections.clone();
    let clone_input_command = input_command.clone();
    let clone_process_id = clone_self.process_id.clone();
    println!("Processo em background [{}] foi iniciado", clone_process_id.clone());
    clone_pipe_sections[last_pipe_index].args.remove(last_arg_index);
    thread::spawn(move || {
      clone_self.execute(&clone_pipe_sections);
      print!("Processo em background [{}] executado {}\n", clone_process_id, clone_input_command);
      clone_self.show_path()
    });
    self.process_id += 1;
  }

  pub fn execute_batch(&mut self, pipe_sections: &Vec<Command>) {
    if pipe_sections.len() > 1 {
      println!("Mais de uma pipe_section");
    } else {
      if pipe_sections[0].args.len() > 0 {
        println!("Não passar argumento ao executar o  programa");
      } else {
        let filename = pipe_sections[0].command_name.replace("./", "");
        let lines = parser_helper::parse_batch_program(file_helper::read_file(filename));
        for mut line in lines {
          let pipe_sections: Vec<Command> = parser_helper::parse_commandline(line.clone());
          self.sync_async_execute(&pipe_sections, &line);
          line.push_str("\n");
          self.enqueue_command(line);
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
    
    self.enqueue_command(command.clone());
    
    let pipe_sections: Vec<Command> = self.parse_command(command.clone());
    if pipe_sections[0].command_name.eq("exit") {
      return
    }

    if pipe_sections[0].command_name.contains("./"){
      self.execute_batch(&pipe_sections);
    } else{
      self.sync_async_execute(&pipe_sections, &command)
    }
    
    self.run();
  }

}
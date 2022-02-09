use std::io;
use std::io::Write;
use crate::domain::model::command::Command;
use crate::infra::os_manager;
use crate::utils::parser_helper;
use crate::utils::file_helper;

#[derive(Clone)]
pub struct BashManager {
  pub username: String,
  pub history: Vec<String>,
  pub paths: Vec<String>
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
      match section.command_name.as_str() {
        "cd" => os_manager::cd(section.args.clone()),
        "ver" => os_manager::ver(section.args.clone()),
        "history" => os_manager::history(section.args.clone(), (*self).clone()),
        _ => {
          if section.args.iter().any(|i| i=="<" || i==">" || i==">>") {
            print!("{}", os_manager::redir(section.clone(), (*self).clone()));
            return
          }
          let output = os_manager::execute_command(section.clone(), (*self).clone());
          file_helper::delete_file(home.clone());
          // println!("{:#?}", section.args);
          // print!("{}", output);
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

  pub fn run(&mut self) {
    self.show_path();
    let command = self.read_command();
    if command.len() == 1 {
      self.run();
      return
    }

    let mut pipe_sections: Vec<Command> = self.parse_command(command.clone());
    if pipe_sections[0].command_name.eq("exit") {
      return
    }
    
    self.execute(&mut pipe_sections);
    self.enqueue_command(command);
    self.run();
  }

}
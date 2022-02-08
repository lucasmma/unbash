use std::io;
use std::io::Write;
use crate::domain::model::command::Command;
use crate::infra::os_manager;
use crate::utils::parser_helper;

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

  pub fn execute(&self, pipe_sections: Vec<Command>){
    for section in pipe_sections.clone() {
      match section.command_name.as_str() {
        "cd" => os_manager::cd(section.args),
        "ver" => os_manager::ver(section.args),
        "history" => os_manager::history(section.args, (*self).clone()),
        _ => {
          os_manager::execute_command(section, (*self).clone());
        }
      }
    }
  }

  pub fn parse_command(&mut self, command: String) -> Vec<Command> {
    let command: Vec<Command> = parser_helper::parse_commandline(command);
    command
  }

  pub fn run(&mut self) {
    self.show_path();
    let command = self.read_command();
    if command.len() == 0{
      self.run();
      return
    }

    let pipe_sections: Vec<Command> = self.parse_command(command.clone());
    if pipe_sections[0].command_name.eq("exit") {
      return
    }
    
    self.execute(pipe_sections);
    self.enqueue_command(command);
    self.run();
  }

}
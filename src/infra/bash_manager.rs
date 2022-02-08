use std::io;
use std::io::Write;
use crate::domain::model::command::Command;
use crate::infra::os_manager;

#[path = "../utils/parser_helper.rs"] mod parser_helper;

#[derive(Clone)]
pub struct BashManager {
  pub username: String,
  pub history: Vec<String>
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
    let initial_section = pipe_sections[0].clone();
    match initial_section.command_name.as_str() {
      "cd" => os_manager::cd(initial_section.args),
      "ls" => os_manager::ls(initial_section.args),
      "sleep" => os_manager::sleep(initial_section.args),
      "cat" => os_manager::cat(initial_section.args),
      "mkdir" => os_manager::mkdir(initial_section.args),
      "echo" => os_manager::echo(initial_section.args),
      "history" => os_manager::history(initial_section.args, (*self).clone()),
      _ => println!("qualquer coisa")
    }
  }

  pub fn parse_command(&mut self, command: String) -> Vec<Command> {
    let command: Vec<Command> = parser_helper::parse_commandline(command);
    command
  }

  pub fn run(&mut self) {
    self.show_path();
    let command = self.read_command();
    let pipe_sections: Vec<Command> = self.parse_command(command.clone());

    if pipe_sections[0].command_name.eq("exit") {
      return
    }
    
    self.execute(pipe_sections);
    self.enqueue_command(command);
    self.run();
  }

}
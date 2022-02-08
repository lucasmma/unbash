use std::io;
use std::io::Write;
use crate::domain::model::command::Command;
use crate::infra::os_manager;

#[path = "../utils/parser_helper.rs"] mod parser_helper;
pub struct BashManager {
  pub username: String
}

impl BashManager {
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
      _ => println!("qualquer coisa")
    }
  }

  pub fn run(&self) {
    self.show_path();
    let command = self.read_command();
    let pipe_sections: Vec<Command> = parser_helper::parse_commandline(command);
    if pipe_sections[0].command_name.eq("exit") {
      return
    }
    self.execute(pipe_sections);
    self.run();
  }

}
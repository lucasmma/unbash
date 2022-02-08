use std::env;
use std::io;
use std::io::Write;
use crate::domain::model::command::Command;

#[path = "../utils/parser_helper.rs"] mod parser_helper;

pub struct BashManager {
  pub username: String
}

impl BashManager {
  pub fn show_path(&self) {
    let result =  env::current_dir();
    match result {
      Ok(v) => {
        print!("Unb - {} - {} ", self.username, v.display());
        io::stdout().flush().map_err(|err| println!("{:?}", err)).ok();
      },
      Err(_e) => print!("Unb - {} - not-found ", self.username),
    }
  }

  pub fn read_command(&self) -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|err| println!("{:?}", err)).ok();
    input
  }

  pub fn execute(&self, pipe_sections: Vec<Command>){
    // env::set_current_dir("../");
  }

  pub fn run(&self) {
    self.show_path();
    let command = self.read_command();
    let pipe_sections: Vec<Command> = parser_helper::parse_line(command);
    if pipe_sections[0].command_name.eq("exit") {
      return
    }
    self.execute(pipe_sections);
    self.run();
  }

}
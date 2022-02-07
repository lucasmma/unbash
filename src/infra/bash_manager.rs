use std::env;
use std::io;
use std::io::Write;

pub struct BashManager {
  pub username: String
}

impl BashManager {
  pub fn show_path(&self) {
    let result =  env::current_dir();
    match result {
      Ok(v) => {
        print!("Unb - {} - {} ", self.username, v.display());
        io::stdout().flush();
      },
      Err(e) => println!("Erro: {}", e),
    }
  }

  pub fn read_command(&self) -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    input
  }

  pub fn run(&self) {
    self.show_path();
    let command = self.read_command();
  }

}
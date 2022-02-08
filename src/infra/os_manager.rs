use std::{thread, time};
use std::fs;
use std::io::Error;
use crate::infra::bash_manager::BashManager;
use crate::domain::model::command::Command;

pub fn get_current_directory()-> String {
  let current_path = std::env::current_dir().unwrap();
  current_path.display().to_string()
}

pub fn cd(args: Vec<String>) {
  if args.len() == 0 {
    std::env::set_current_dir("/").map_err(|_err| println!("Diretório não disponível")).ok();
  } else if args.len() > 1 {
    println!("Comando com mais de um argumento");
  } else{
    std::env::set_current_dir(args[0].as_str()).map_err(|_err| println!("Diretório não disponível")).ok();
  }
}

pub fn sleep(args: Vec<String>) {
  if args.len() == 0 {
    println!("Comando sem argumentos");
  } else if args.len() > 1 {
    println!("Comando com mais de um argumento");
  } else {
    match args[0].parse::<i64>() {
      Ok(num) => {
        let millis = time::Duration::from_millis((num * 1000).try_into().unwrap());
        thread::sleep(millis);
      },
      Err(_e) => {
        println!("Argumento não é um número")
      },
    }    
  }
}

pub fn list_directory(path: &str) {
  for file in fs::read_dir(path).unwrap() {
    println!("{}", file.unwrap().path().display());
  }
}

pub fn ls(args: Vec<String>) {
  if args.len() == 0 {
    list_directory(get_current_directory().as_str());
  } else if args.len() > 1 {
    println!("Comando com mais de um argumento");
  } else{
    list_directory(args[0].as_str());
  }
}

pub fn cat(args: Vec<String>) {
  if args.len() == 0 {
    println!("Nenhum argumento no comando");
  } else if args.len() > 1 {
    println!("Comando com mais de um argumento");
  } else{
    let read_file = || -> Result<(), Error> {
      let contents = fs::read_to_string(args[0].as_str())?;
      println!("{}", contents);
      Ok(())
    };

    if let Err(_err) = read_file() {
      println!("Nenhum arquivo encontrado");
    }
  }
}

pub fn mkdir(args: Vec<String>) {
  if args.len() == 0 {
    println!("Nenhum argumento no comando");
  } else if args.len() > 1 {
    println!("Comando com mais de um argumento");
  } else{
    fs::create_dir(args[0].as_str()).unwrap();
  }
}

pub fn inline_args(args: Vec<String>) -> String{
  let mut phrase : String = String::from("");
  for arg in args {
    phrase.push_str(&arg);
    phrase.push_str(" ");
  }
  phrase
}

pub fn echo(args: Vec<String>) {
  let phrase : String = inline_args(args);
  println!("{}", phrase);
}

pub fn history(args: Vec<String>, mut bash: BashManager) {
  if args.len() == 0 {
    for commands in bash.history.clone() {
      print!("{}", commands);
    }
  } else if args.len() > 1 {
    println!("Comando com mais de um argumento");
  } else{
    match args[0].parse::<usize>() {
      Ok(num) => {
        if num > 0 && num <= 10 && num < bash.history.len() {
          let command: Vec<Command> = bash.parse_command(bash.history[num-1].clone());
          bash.execute(command);
        } else{
          println!("Número fora da range");
        }
      },
      Err(_e) => {
        println!("Argumento não é um número")
      },
    }; 
  }
}

pub fn ver(args: Vec<String>) {
  if args.len() == 0 {
    println!("Versão 1.0.0 - 08/02/22 - lucasmma")
  } else {
    println!("Comando não deve ser inserido com argumento");
  }
}
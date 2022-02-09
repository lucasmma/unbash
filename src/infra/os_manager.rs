use crate::infra::bash_manager::BashManager;
use crate::domain::model::command::Command;
use crate::utils::file_helper;
use std::process;
use home;
use std::io::Error;


pub fn get_current_directory()-> String {
  let current_path = std::env::current_dir().unwrap();
  current_path.display().to_string()
}

pub fn get_home_directory() -> String {
  let mut home_path = String::from("");
  match home::home_dir() {
    Some(path) => home_path = path.display().to_string() ,
    None => println!("Impossible to get your home dir!"),
  }
  home_path
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
          let mut command: Vec<Command> = bash.parse_command(bash.history[num-1].clone());
          bash.execute(&mut command);
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

pub fn ver(args: Vec<String>){
  if args.len() == 0 {
    println!("Versão 1.0.0 - 08/02/22 - lucasmma")
  } else {
    println!("Comando não deve ser inserido com argumento");
  }
}

pub fn execute_command(command: Command, bash: BashManager)-> String {
  for path in bash.paths {
    let mut full_path : String = path.clone();
    if path.chars().nth(path.len()-1).unwrap() != '/' {
      full_path.push_str("/");
    }
    full_path.push_str(command.command_name.as_str());
    if file_helper::file_exists(full_path.clone()) {
      let output = || -> Result<process::Output, Error> {
        Ok(process::Command::new(full_path).args(command.args.clone()).output()?)
      };
      
    
      match output() {
        Ok(v) => {
          // println!("{:#?}", v.stdout);
          // file_helper::create_write_file(String::from("ola.txt"), String::from_utf8_lossy(&v.stdout).to_string());
          return String::from_utf8(v.stdout).unwrap()
        },
        Err(_err) => {
          return String::from("")
        }
      }
    }
  }

    return String::from("Não achei o comando")
}

pub fn redir(mut section: Command, bash: BashManager)-> String{
  let index = section.args.iter().position(|x| *x == "<" || *x == ">" || *x == ">>").unwrap();
  if section.args.iter().any(|i| i=="<") {
    section.args.remove(index);
    return execute_command(section, bash);
  } else {
    if index < section.args.len() - 1 {
      let filename = section.args[index + 1].clone();
      let signal = section.args[index].clone();
      section.args.remove(index + 1);
      section.args.remove(index);
      let output = execute_command(section.clone(), bash);
      if output.eq("Não achei o comando"){
        return "Não achei o comando".to_string()
      } else if output.eq(""){
        return "".to_string()
      } else{
        if signal.eq(">") {
          file_helper::create_write_file(filename, output);
          return "".to_string()
        } else {
          file_helper::append_file(filename, output);
          return "".to_string()
        }
      }
    } else {
      return "Argumentos inválidos".to_string()
    }
  } 
}
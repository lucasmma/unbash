use crate::infra::bash_manager::BashManager;
use crate::domain::model::command::Command;
use crate::utils::file_helper;
use std::process;
use home;
use std::io::Error;
use std::io::Write;


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

pub fn cd(args: Vec<String>) -> String {
  if args.len() == 0 {
    std::env::set_current_dir("/").map_err(|_err| String::from("Diretório não disponível\n")).ok();
  } else if args.len() > 1 {
    return String::from("Comando com mais de um argumento\n");
  } else{
    std::env::set_current_dir(args[0].as_str()).map_err(|_err| String::from("Diretório não disponível\n")).ok();
  }
  return String::from("")
}

pub fn history(args: Vec<String>, mut bash: BashManager) -> String {
  if args.len() == 0 {
    let mut commands : String = "".to_owned();
    for command_lines in bash.history.clone() {
      commands.push_str(command_lines.as_str());
      commands.push_str("\n");
    }
    return commands
  } else if args.len() > 1 {
    return String::from("Comando com mais de um argumento\n");
  } else{
    match args[0].parse::<usize>() {
      Ok(num) => {
        if num > 0 && num <= 10 && num < bash.history.len() {
          let mut command: Vec<Command> = bash.parse_command(bash.history[num-1].clone());
          bash.execute(&mut command);
          return String::from("")
        } else{
          return String::from("Número fora da range\n")
        }
      },
      Err(_e) => {
        return String::from("Argumento não é um número\n")
      },
    }; 
  }
}

pub fn ver(args: Vec<String>) -> String{
  if args.len() == 0 {
    return String::from("Versão 1.0.0 - 08/02/22 - lucasmma")
  } else {
    return String::from("Comando não deve ser inserido com argumento");
  }
}

pub fn execute_command(command: Command, bash: BashManager, stdin: String)-> String {
  for path in bash.paths {
    let mut full_path : String = path.clone();
    if path.chars().nth(path.len()-1).unwrap() != '/' {
      full_path.push_str("/");
    }
    full_path.push_str(command.command_name.as_str());
    if file_helper::file_exists(full_path.clone()) {
      let process = || -> Result<std::process::Child, Error> {
        let mut child = process::Command::new(full_path);
        child.args(command.args.clone());
        child.stdin(std::process::Stdio::piped());
        child.stdout(std::process::Stdio::piped());
        Ok(child.spawn()?)
      };
      
    
      match process() {
        Ok(mut child) => {
          if stdin.len() > 0 {
            let stdin_child = child.stdin.as_mut().unwrap();
            match stdin_child.write_all(stdin.clone().as_bytes()) {
              Err(e) => return e.to_string(),
              _ => ()
            }
            drop(stdin_child);
          }
          match child.wait_with_output() {
            Ok(output) => {
              return String::from_utf8_lossy(&output.stdout).to_string();
            },
            Err(e) => return e.to_string(),
          };
        },
        Err(err) => {
          return err.to_string()
        }
      }
    }
  }

  return String::from("Não achei o comando\n")
}

pub fn redir(mut section: Command, bash: BashManager)-> String{
  let index = section.args.iter().position(|x| *x == "<" || *x == ">" || *x == ">>").unwrap();
  if section.args.iter().any(|i| i=="<") {
    section.args.remove(index);
    return execute_command(section, bash, "".to_string());
  } else {
    if index < section.args.len() - 1 {
      let filename = section.args[index + 1].clone();
      let signal = section.args[index].clone();
      section.args.remove(index + 1);
      section.args.remove(index);
      let output = execute_command(section.clone(), bash, "".to_string());
      if output.clone().replace("\n", "").eq("Não achei o comando"){
        return "Não achei o comando\n".to_string()
      } else if output.clone().replace("\n", "").eq("Erro"){
        return "Erro\n".to_string()
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
      return "Argumentos inválidos\n".to_string()
    }
  } 
}
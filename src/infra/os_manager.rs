use std::{thread, time};
use std::fs;
use std::io::Error;

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
    let number: i64 = args[0].parse::<i64>().unwrap().try_into().unwrap();
    let millis = time::Duration::from_millis((number * 1000).try_into().unwrap());
    thread::sleep(millis);
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
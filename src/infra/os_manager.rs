use std::{thread, time};

pub fn cd(args: Vec<String>) {
  if args.len() == 0 {
    std::env::set_current_dir("/").map_err(|_err| println!("Diretório não disponível")).ok();
  } else if args.len() > 1 {
    println!("Comando mais de um argumento");
  } else{
    std::env::set_current_dir(args[0].as_str()).map_err(|_err| println!("Diretório não disponível")).ok();
  }
}

pub fn sleep(args: Vec<String>) {
  if args.len() == 0 {
    println!("Comando sem argumentos");
  } else if args.len() > 1 {
    println!("Comando mais de um argumento");
  } else {
    let number: i64 = args[0].parse::<i64>().unwrap().try_into().unwrap();
    let millis = time::Duration::from_millis((number * 1000).try_into().unwrap());
    thread::sleep(millis);
  }
}
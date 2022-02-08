pub fn cd(args: Vec<String>) {
  if args.len() == 0 {
    std::env::set_current_dir("/").map_err(|_err| println!("Diretório não disponível")).ok();
  } else if args.len() > 1 {
    println!("Comando mais de um argumento");
  } else{
    std::env::set_current_dir(args[0].as_str()).map_err(|_err| println!("Diretório não disponível")).ok();
  }
}
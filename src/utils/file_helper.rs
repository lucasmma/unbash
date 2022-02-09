use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::Write;

pub fn read_file(filename: String) -> String{
  let file_reader = || -> Result<String, Error> {
    let contents = fs::read_to_string(filename.as_str())?;
    Ok(contents.to_string())
  };

  match file_reader() {
    Ok(v) => return v,
    Err(_err) => return String::from("anything" )
  }
}

pub fn create_file(filename: String){
  let file = || -> Result<(), Error> {
    File::create(filename)?;
    Ok(())
  };

  if let Err(_err) = file() {

  }
}

pub fn create_write_file(filename: String, content: String){
  let file = || -> Result<(), Error> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
  };

  if let Err(_err) = file() {

  }
}

pub fn delete_file(filename: String) {
  if fs::metadata(filename.clone()).is_ok(){
    let file = || -> Result<(), Error> {
      fs::remove_file(filename)?;
      Ok(())
    };
  
    if let Err(_err) = file() {
  
    }
  }
}
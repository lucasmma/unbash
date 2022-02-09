#[derive(Clone)]
#[derive(PartialEq)]
pub struct Command {
  pub command_name: String,
  pub args: Vec<String>
}
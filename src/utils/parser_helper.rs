use crate::domain::model::command::Command;

pub fn parse_line(line: String) -> Vec<Command>{
  let pipe_sections = line.split("|");
  let mut pipe_sections_parsed: Vec<Command> = vec![];
  for pipe_section in pipe_sections {
    let mut commands: Vec<String> = pipe_section.split_whitespace().map(|s| s.to_string()).collect();
    let command_name: String = commands[0].clone();
    commands.remove(0);
    pipe_sections_parsed.push(Command { command_name: command_name.to_string(), args: commands});
  }

  return pipe_sections_parsed;
}
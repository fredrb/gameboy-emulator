use std::io::{Write, stdout, stdin};

pub enum CommandType {
  Breakpoint,
  ShowRegister,
  ShowMemory,
  Local,
  Print,
  Next,
  Continue,
  Start,
  Log,
  Unkown
}

pub struct InputCommand {
  pub class: CommandType,
  pub args: Vec<String>
}

impl InputCommand {
  pub fn new(class: CommandType, args: Vec<String>) -> Self {
    InputCommand { class, args }
  }

  pub fn from_class(class: &str, raw: Vec<&str>) -> Self {
    // println!("read {}", class);
    let command_type = match class {
      "break" | "b" => CommandType::Breakpoint,
      "reg" | "r" => CommandType::ShowRegister,
      "mem" | "m" => CommandType::ShowMemory,
      "local" | "lc" => CommandType::Local,
      "print" | "p" => CommandType::Print,
      "next" | "n" => CommandType::Next,
      "continue" | "c" => CommandType::Continue,
      "start" | "s" => CommandType::Start,
      "log" | "l" => CommandType::Log,
      _ => CommandType::Unkown,
    };
    let args: Vec<String> = raw.into_iter().map(|x| String::from(x)).collect();
    return InputCommand::new(command_type, args);
  }
}

pub struct DebuggerInput {
  pub history: Vec<String>,
}

impl DebuggerInput {
  pub fn new() -> Self {
    DebuggerInput { history: vec!() }
  }

  pub fn poll_command() -> InputCommand {
    print!("> ");
    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut parts: Vec<&str> = input.rsplit(' ').collect();
    parts = parts.into_iter().map(|x| x.trim()).rev().collect();
    let command_word = parts[0].clone();
    // println!("{}", parts.len());
    // for l in 0..parts.len() {
    //   println!("{}: {}", l, parts[l])
    // }
    parts.remove(0);
    InputCommand::from_class(command_word, parts)
  }
}
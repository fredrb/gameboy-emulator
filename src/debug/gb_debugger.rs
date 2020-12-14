use crate::debug::debugger::Debuggable;
use crate::gameboy::Gameboy;

use super::{input::{DebuggerInput, InputCommand}, ui::{MessageType}};

pub struct DebuggerState {
  pub breakpoints: Vec<u16>,
  pub break_next: bool,
  pub log_next: bool,
}

impl DebuggerState {
  pub fn new() -> Self {
    DebuggerState { 
      breakpoints: vec!(),
      break_next: false,
      log_next: false,
    }
  }

  pub fn args_to_u16(&self, command: &InputCommand) -> u16 {
    return match command.args.get(0) {
      Some(n) => i64::from_str_radix(n, 16).unwrap() as u16,
      None => 0x00000,
    };
  }

  pub fn add_breakpoint(&mut self, command: &InputCommand) -> u16 {
    let addr = self.args_to_u16(command);
    self.breakpoints.push(addr as u16);
    addr
  }
}

impl Debuggable for Gameboy {

  fn on_started(&mut self) {
    self.terminal.print_message(MessageType::Normal, "Debugger started");
    self.terminal.print_message(MessageType::Important, " - Add breakpoints (b)");
    self.terminal.print_message(MessageType::Important, " - Add start (s)");

    let should_start = false;
    while !should_start {
      let command = DebuggerInput::poll_command();
      match command.class {
          super::input::CommandType::Breakpoint => {
            let addr = self.state.add_breakpoint(&command);
            self.terminal.print_message(MessageType::Good, &format!("Breakpoint added @ {:#04x}", addr));
          },
          super::input::CommandType::Start => break,
          _ => self.terminal.print_message(MessageType::Bad, "Command not allowed")
      }
    }
  }

  fn on_breakpoint(&mut self, addr: u16) {
    self.terminal.print_message(MessageType::Good, &format!("Application stopped at breakpoint: {:#06x}", addr));
    let mut escape = false;
    while !escape {
      let command = DebuggerInput::poll_command();
      match command.class {
          super::input::CommandType::Breakpoint => {
            let addr = self.state.add_breakpoint(&command);
            self.terminal.print_message(MessageType::Good, &format!("Breakpoint added @ {:#04x}", addr));
          },
          super::input::CommandType::ShowRegister => self.terminal.print_registers(&self.cpu.reg),
          super::input::CommandType::ShowMemory => {
            let since = self.state.args_to_u16(&command) as usize;
            self.terminal.print_memory(since - 5, &self.mmu.data[(since-5)..(since+5)], 5);
          },
          super::input::CommandType::Local => {
            let lower_bound = match self.cpu.reg.pc {
              0..=19 => 0,
              x => x - 20,
            };
            let center = self.cpu.reg.pc - lower_bound;
            self.terminal.print_hextable(&self.mmu.data[lower_bound..self.cpu.reg.pc+20], 
              center, 0)
          },
          super::input::CommandType::Print => self.terminal.print_message(MessageType::Bad, "Command not allowed"),
          super::input::CommandType::Next => {
            self.state.break_next = true;
            escape = true;
          },
          super::input::CommandType::Log => {
            match command.args.get(0) {
              Some(s) => {
                match s.as_str() {
                  "next" | "n" | "on" => self.state.log_next = true,
                  "off" => self.state.log_next = false,
                  _ => self.state.log_next = false
                }
              }
              None => self.state.log_next = true,
            };
            self.terminal.print_message(MessageType::Good, &format!("Logger set to '{}'", if self.state.log_next { "on" } else {"off"}));
          },
          super::input::CommandType::Continue => {
            self.state.break_next = false;
            escape = true;
          },
          super::input::CommandType::Start => self.terminal.print_message(MessageType::Bad, "Command not allowed at this stage"),
          super::input::CommandType::Unkown => self.terminal.print_message(MessageType::Bad, "Unkown command")
      }
    }
  }

  fn should_stop(&self, addr: u16) -> bool {
    return self.state.breakpoints.contains(&addr) || self.state.break_next;
  }
}
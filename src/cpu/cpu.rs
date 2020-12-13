extern crate config;

use crate::cpu::decoder::OpcodeDecoder;
use crate::{debug::logger::LoggableComponent};
use crate::reg::{Registers};
// use crate::opcode::OpCodes;
use crate::mmu::VirtualMemory;
use crate::debug::logger::{LogMessage,LogEvents};

pub struct CPU {
  pub reg: Registers,
  pub log_enabled: bool,
  message_buffer: Vec<LogMessage>,
}

impl CPU {
  pub fn new(log: bool) -> Self {
    CPU {
      reg: Registers::new(),
      log_enabled: log,
      message_buffer: vec!(),
    }
  }

  pub fn log(&mut self, event: LogEvents, message: String) {
    self.message_buffer.push((event, message));
  }

  pub fn tick(&mut self, bus: &mut VirtualMemory) {
    self.message_buffer.clear();
    let pc = self.reg.pc.clone();
    self.log(LogEvents::Tick, format!("[TICK] ADDR({:#06x})", pc));
    let mut processor = OpcodeDecoder{
      bus,
      reg: &mut self.reg,
      message_buffer: &mut self.message_buffer
    };
    processor.run_opcode();
  }
}

impl LoggableComponent for CPU {
  fn dump_log_messages(&mut self) -> Vec<LogMessage> {
    let messages = self.message_buffer.clone();
    self.message_buffer.clear();
    return messages;
  }
}
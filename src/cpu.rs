extern crate config;

use crate::debug::logger::LoggableComponent;
use crate::{Cartridge, opcodes::{OpCodeSuccess}};
use crate::reg::{Registers,Flag};
use crate::opcodes::OpCodes;
use crate::mmu::VirtualMemory;
use crate::mmu;
use crate::debug::logger::{LogMessage,LogEvents};

pub struct CPU {
  pub reg: Registers,
  pub cartridge: Option<Cartridge>,
  pub log_enabled: bool,
  message_buffer: Vec<LogMessage>,
}

impl CPU {
  pub fn new(log: bool) -> Self {
    CPU {
      reg: Registers::new(),
      cartridge: None,
      log_enabled: log,
      message_buffer: vec!(),
    }
  }

  pub fn load_cartridge(&mut self, cartridge: Cartridge) {
    println!("Loading cartridge into GameBoy");
    self.cartridge = Some(cartridge);
  }

  fn increment(&mut self) {
    self.reg.pc += 1;
  }

  pub fn fetch_post_increment(&mut self, bus: &mut VirtualMemory) -> u8 {
    let op_code = bus.fetch(self.reg.pc);
    self.increment();
    return op_code;
  }

  pub fn set_pc(&mut self, byte: usize) {
    self.log(LogEvents::Register, format!("[SET_PC]: {:#06x} -> {:#06x}", self.reg.pc, byte));
    self.reg.pc = byte;
  }

  pub fn log(&mut self, event: LogEvents, message: String) {
    self.message_buffer.push((event, message));
  }

  pub fn tick(&mut self, bus: &mut VirtualMemory) {
    self.message_buffer.clear();
    let pc = self.reg.pc.clone();
    let instruction = self.fetch_post_increment(bus);
    self.log(LogEvents::Tick, format!("[TICK] ADDR({:#06x})", pc));
    self.run_opcode(instruction, bus);
  }
}

impl LoggableComponent for CPU {
  fn dump_log_messages(&mut self) -> Vec<LogMessage> {
    let messages = self.message_buffer.clone();
    self.message_buffer.clear();
    return messages;
  }
}
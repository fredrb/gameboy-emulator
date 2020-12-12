use crate::debug::logger::LogMessage;
use crate::cpu::CPU;
use crate::mmu::VirtualMemory;
use crate::gb_config::gb_config;
use crate::external::cartridge::Cartridge;
use crate::external::boot_rom_loader;
use crate::reg::RegCode;
use crate::debug::logger::{LoggerClient, LogEvents, LoggableComponent};

pub struct Gameboy {
  pub cpu: CPU,
  pub mmu: VirtualMemory,
  pub cfg: gb_config,
  pub cartridge: Option<Cartridge>,

  logger_client: LoggerClient,
}

// pub struct StateChange<T,V> {
//   state_type: T,
//   new_value: V
// }

// trait StateChangeable<T,V> {
//   fn apply_state(&mut self, entry: &StateChange<T,V>);
// }

pub trait ExternalHook {
  fn when_started(gb: &mut Gameboy);
  fn before_tick(gb: &mut Gameboy);
  fn after_tick(gb: &mut Gameboy);
}

// impl StateChangeable<RegCode,u8> for Gameboy {
//   fn apply_state(&mut self, c: &StateChange<RegCode,u8>) {

//   }
// }

impl Gameboy {
  pub fn new(cfg: gb_config, logger_client: LoggerClient) -> Self {
    Gameboy {
      cpu: CPU::new(false),
      mmu: VirtualMemory::new(),
      cartridge: None,
      cfg,
      logger_client
    }
  }

  pub fn load_cartridge(&mut self, cartridge: Cartridge) {
    self.logger_client.send((LogEvents::Initializing, String::from("Initializing ROM")));
    match self.mmu.load_rom(cartridge.content.as_slice()) {
      Ok(size) => self.logger_client.send((LogEvents::Initializing, format!("{} bytes loaded into memory", size))),
      Err(why) => panic!("Failed to load ROM {}", why) // @TODO HANDLE PANICS
    }
    
    self.cartridge = Some(cartridge);
    if self.cfg.boot_rom_enabled {
      let boot_rom = boot_rom_loader::load_boot_rom(&self.cfg.boot_rom_path);
      match self.mmu.load_rom(boot_rom.as_slice()) {
        Ok(size) => self.logger_client.send((LogEvents::Initializing, format!("{} bytes loaded into boot rom area", size))),
        Err(why) => panic!("Failed to load boot rom {}", why)
      }
    } else {
      self.cpu.reg.pc = 0x100;
    }
  }

  pub fn start(&mut self) {
    self.logger_client.send((LogEvents::Initializing, String::from("Starting GB main loop")));
    loop {
      let mut messages: Vec<LogMessage> = vec!();
      self.cpu.tick(&mut self.mmu);
      messages.extend(self.cpu.dump_log_messages());
      messages.extend(self.mmu.dump_log_messages());
      for m in messages {
        self.logger_client.send(m);
      }

      for s in self.cpu.reg.print_registers() {
        self.logger_client.send((LogEvents::Register, format!("\t[REG] {}", s)));
      }
    }
  }
}
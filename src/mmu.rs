use crate::debug::logger::LogMessage;
use crate::debug::logger::LogEvents;
use crate::debug::logger::LoggableComponent;
use std::{thread, time};

// pub struct MemoryBus {
//   memory: VirtualMemory,
// }

// impl MemoryBus {
//   pub fn read(&self, addr: usize) -> Result<u8,String> {
//     let ten_millis = time::Duration::from_millis(10);
//     thread::sleep(ten_millis);
//     let byte = self.memory.data[addr];
//     Ok(byte)
//   }

//   pub fn write(&mut self, addr: usize, byte: u8) -> Result<usize,String> {
//     self.memory.data[addr] = byte;
//     if addr > 0x5000 {
//       println!("WRITE [{:#06x}]: {:#04x}", addr, byte);
//     }
//     Ok(addr)
//   }
// }

pub struct VirtualMemory {
  pub data: Vec<u8>,
  message_buffer: Vec<LogMessage>
}

impl VirtualMemory {

  pub fn from_rom(raw: &[u8]) -> Result<Self,String> {
    if raw.len() >= 0xFFFF {
      return Err(String::from("Rom is bigger than target system memory"))
    }
    let mut data = vec![0;0xFFFF];
    data[0..raw.len()].copy_from_slice(&raw);
    Ok(VirtualMemory {data, message_buffer: vec!() })
  }

  pub fn new() -> Self {
    VirtualMemory {
      data: vec![0;0xFFFF],
      message_buffer: vec!()
    }
  }

  pub fn load_rom(&mut self, raw: &[u8]) -> Result<usize,String> {
    if raw.len() >= 0xFFFF {
      return Err(String::from("Rom is bigger than target system memory"))
    }
    self.data[0..raw.len()].copy_from_slice(&raw);
    Ok(raw.len())
  }

  pub fn fetch(&mut self, pointer: usize) -> u8 {
    // let ten_millis = time::Duration::from_millis(1);
    // thread::sleep(ten_millis);
    let byte = self.data[pointer];
    self.message_buffer.push((LogEvents::MemoryFetch, format!("[FETCH] ADDR({:#06x}): {:#04x}", pointer, byte)));
    // println!("READ [{:#06x}]: {:#04x}", pointer, byte);
    return byte;
  }

  pub fn save(&mut self, addr: usize, byte: u8) -> Result<(),String> {
    self.data[addr] = byte;
    let log = match addr {
      0x8000..=0x9FFF => (LogEvents::VramSave, format!("[VRAM_SAVE] ADDR({:#06x}): {:#04x}", addr, byte)),
      _ => (LogEvents::MemorySave, format!("[SAVE] ADDR({:#06x}): {:#04x}", addr, byte)),
    };
    self.message_buffer.push(log);
    return Ok(());
  }
}

impl LoggableComponent for VirtualMemory {
  fn dump_log_messages(&mut self) -> Vec<(LogEvents, String)> { 
    let messages = self.message_buffer.clone();
    self.message_buffer.clear();
    return messages;
  }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn should_copy_content_into_virtual_memory() {
      let rom = vec![0xFF;0x4000];
      match VirtualMemory::from_rom(&rom.as_slice()) {
        Ok(mmu) => {
          assert_eq!(mmu.data.len(), 0xFFFF);
          assert_eq!(mmu.data[0x0], 0xFF);
          assert_eq!(mmu.data[0xFF], 0xFF);
          assert_eq!(mmu.data[0x3999], 0xFF);
          assert_eq!(mmu.data[0x4000], 0x0);
          assert_eq!(mmu.data[0x4001], 0x0);
        }
        _ => panic!("Failed")
      }
    }
}
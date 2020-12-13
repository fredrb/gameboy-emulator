use crate::{debug::logger::LogMessage, mmu::VirtualMemory, reg::RegCode, reg::{Flag, Registers}};
use crate::cpu::opcode::*;

pub struct OpcodeDecoder<'tick> {
  pub bus: &'tick mut VirtualMemory,
  pub reg: &'tick mut Registers,
  pub message_buffer: &'tick mut Vec<LogMessage>
}

impl OpcodeDecoder<'_> {
  fn read_next_8bit(&mut self) -> u8 {
    return self.fetch_post_increment();
  }

  fn read_next_16bits(&mut self) -> (u8, u8) {
    let right_byte = self.fetch_post_increment();
    let left_byte = self.fetch_post_increment();
    return (left_byte, right_byte);
  }

  fn read_next_addr(&mut self) -> usize {
    let b = self.read_next_16bits();
    return (((b.0 as u16) << 8) & (b.1 as u16)) as usize;
  }

  pub fn fetch_post_increment(&mut self) -> u8 {
    let op_code = self.bus.fetch(self.reg.pc);
    self.reg.inc_pc();
    return op_code;
  }

  pub fn run_opcode(&mut self) {
    let opcode = self.fetch_post_increment();
    match opcode {
      0x00 => (),
      0x01 | 0x11 | 0x21 | 0x31 => {
        let byte = self.read_next_16bits();
        match opcode {
          0x01 => self.ld(RegCode::BC, byte),
          0x11 => self.ld(RegCode::DE, byte),
          0x21 => self.ld(RegCode::HL, byte),
          0x31 => self.ld(RegCode::SP, byte),
          _ => ()
        }
      }

      0x02 => self.ld(self.reg.get_16bit(&RegCode::BC) as usize, RegCode::A),
      0x12 => self.ld(self.reg.get_16bit(&RegCode::DE) as usize, RegCode::A),
      0x22 => self.ldi(self.reg.get_16bit(&RegCode::HL) as usize, RegCode::A),
      0x32 => self.ldd(self.reg.get_16bit(&RegCode::HL) as usize, RegCode::A),

      0x04 => self.inc(RegCode::B),
      0x14 => self.inc(RegCode::D),
      0x24 => self.inc(RegCode::H),
      0x34 => self.inc(self.reg.get_16bit(&RegCode::HL) as usize),

      0x05 => self.dec(RegCode::B),
      0x15 => self.dec(RegCode::D),
      0x25 => self.dec(RegCode::H),
      0x35 => self.dec(self.reg.get_16bit(&RegCode::HL) as usize),

      0x06 | 0x16 | 0x26 | 0x36 => {
        let byte = self.read_next_8bit();
        match opcode {
          0x06 => self.ld(RegCode::B, byte),
          0x16 => self.ld(RegCode::D, byte),
          0x26 => self.ld(RegCode::H, byte),
          0x36 => self.ld(self.reg.get_16bit(&RegCode::HL) as usize, byte),
          _ => (),
        }
      }

      0x07 => self.rlc(RegCode::A),
      0x17 => self.rl(RegCode::A),

      0x0a => self.ld(RegCode::A, self.reg.get_16bit(&RegCode::BC) as usize),
      0x1a => self.ld(RegCode::A, self.reg.get_16bit(&RegCode::DE) as usize),
      0x2a => self.ldi(RegCode::A, self.reg.get_16bit(&RegCode::HL) as usize),
      0x3a => self.ldd(RegCode::A, self.reg.get_16bit(&RegCode::HL) as usize),

      0x0c => self.inc(RegCode::C),
      0x1c => self.inc(RegCode::E),
      0x2c => self.inc(RegCode::L),
      0x3c => self.inc(RegCode::A),

      0x0e | 0x1e | 0x2e | 0x3e => {
        let byte = self.read_next_8bit();
        match opcode {
          0x0e => self.ld(RegCode::C, byte),
          0x1e => self.ld(RegCode::E, byte),
          0x2e => self.ld(RegCode::L, byte),
          0x3e => self.ld(RegCode::A, byte),
          _ => (),
        }
      }
      

      0x40 => self.ld(RegCode::B, self.reg.b),
      0x41 => self.ld(RegCode::B, self.reg.c),
      0x42 => self.ld(RegCode::B, self.reg.d),
      0x43 => self.ld(RegCode::B, self.reg.e),
      0x44 => self.ld(RegCode::B, self.reg.h),
      0x45 => self.ld(RegCode::B, self.reg.l),
      0x46 => self.ld(RegCode::B, self.reg.get_16bit(&RegCode::HL) as usize),
      0x47 => self.ld(RegCode::B, self.reg.a),
      0x48 => self.ld(RegCode::C, self.reg.b),
      0x49 => self.ld(RegCode::C, self.reg.c),
      0x4A => self.ld(RegCode::C, self.reg.d),
      0x4B => self.ld(RegCode::C, self.reg.e),
      0x4C => self.ld(RegCode::C, self.reg.h),
      0x4D => self.ld(RegCode::C, self.reg.l),
      0x4E => self.ld(RegCode::C, self.reg.get_16bit(&RegCode::HL) as usize),
      0x4F => self.ld(RegCode::C, self.reg.a),

      0x50 => self.ld(RegCode::D, self.reg.b),
      0x51 => self.ld(RegCode::D, self.reg.c),
      0x52 => self.ld(RegCode::D, self.reg.d),
      0x53 => self.ld(RegCode::D, self.reg.e),
      0x54 => self.ld(RegCode::D, self.reg.h),
      0x55 => self.ld(RegCode::D, self.reg.l),
      0x56 => self.ld(RegCode::D, self.reg.get_16bit(&RegCode::HL) as usize),
      0x57 => self.ld(RegCode::D, self.reg.a),
      0x58 => self.ld(RegCode::E, self.reg.b),
      0x59 => self.ld(RegCode::E, self.reg.c),
      0x5A => self.ld(RegCode::E, self.reg.d),
      0x5B => self.ld(RegCode::E, self.reg.e),
      0x5C => self.ld(RegCode::E, self.reg.h),
      0x5D => self.ld(RegCode::E, self.reg.l),
      0x5E => self.ld(RegCode::E, self.reg.get_16bit(&RegCode::HL) as usize),
      0x5F => self.ld(RegCode::E, self.reg.a),

      0x60 => self.ld(RegCode::H, self.reg.b),
      0x61 => self.ld(RegCode::H, self.reg.c),
      0x62 => self.ld(RegCode::H, self.reg.d),
      0x63 => self.ld(RegCode::H, self.reg.e),
      0x64 => self.ld(RegCode::H, self.reg.h),
      0x65 => self.ld(RegCode::H, self.reg.l),
      0x66 => self.ld(RegCode::H, self.reg.get_16bit(&RegCode::HL) as usize),
      0x67 => self.ld(RegCode::H, self.reg.a),
      0x68 => self.ld(RegCode::L, self.reg.b),
      0x69 => self.ld(RegCode::L, self.reg.c),
      0x6A => self.ld(RegCode::L, self.reg.d),
      0x6B => self.ld(RegCode::L, self.reg.e),
      0x6C => self.ld(RegCode::L, self.reg.h),
      0x6D => self.ld(RegCode::L, self.reg.l),
      0x6E => self.ld(RegCode::L, self.reg.get_16bit(&RegCode::HL) as usize),
      0x6F => self.ld(RegCode::L, self.reg.a),

      0x70 => self.ld(self.reg.get_16bit(&RegCode::HL) as usize, self.reg.b),
      0x71 => self.ld(self.reg.get_16bit(&RegCode::HL) as usize, self.reg.c),
      0x72 => self.ld(self.reg.get_16bit(&RegCode::HL) as usize, self.reg.d),
      0x73 => self.ld(self.reg.get_16bit(&RegCode::HL) as usize, self.reg.e),
      0x74 => self.ld(self.reg.get_16bit(&RegCode::HL) as usize, self.reg.h),
      0x75 => self.ld(self.reg.get_16bit(&RegCode::HL) as usize, self.reg.l),
      0x77 => self.ld(self.reg.get_16bit(&RegCode::HL) as usize, self.reg.a),

      0x78 => self.ld(RegCode::A, self.reg.b),
      0x79 => self.ld(RegCode::A, self.reg.c),
      0x7A => self.ld(RegCode::A, self.reg.d),
      0x7B => self.ld(RegCode::A, self.reg.e),
      0x7C => self.ld(RegCode::A, self.reg.h),
      0x7D => self.ld(RegCode::A, self.reg.l),
      0x7E => self.ld(RegCode::A, self.reg.get_16bit(&RegCode::HL) as usize),
      0x7F => self.ld(RegCode::A, self.reg.a),

      0x20 | 0x30 | 0x81 | 0x82 | 0x83 => {
        let byte = self.read_next_8bit();
        match opcode {
          0x20 => self.jr(!self.reg.check_flag(&Flag::Zero), byte),
          0x30 => self.jr(!self.reg.check_flag(&Flag::CarryFlag), byte),
          0x81 => self.jr(true, byte),
          0x82 => self.jr(self.reg.check_flag(&Flag::Zero), byte),
          0x83 => self.jr(self.reg.check_flag(&Flag::CarryFlag), byte),
          _ => ()
        }
      }

      0xC2 | 0xD2 | 0xC3 | 0xCA | 0xDA => {
        let addr = self.read_next_addr();
        match opcode {
          0xC2 => self.jp(!self.reg.check_flag(&Flag::Zero), addr),
          0xD2 => self.jp(!self.reg.check_flag(&Flag::CarryFlag), addr),
          0xC3 => self.jp(true, addr),
          0xCA => self.jp(self.reg.check_flag(&Flag::Zero), addr),
          0xDA => self.jp(self.reg.check_flag(&Flag::CarryFlag), addr),
          _ => (),
        }
      }

      0xE0 | 0xF0 => {
        let byte = self.read_next_8bit();
        match opcode {
          0xE0 => self.ldh(byte as usize, RegCode::A),
      0xF0 => self.ldh(RegCode::A, byte as usize),
          _ => (),
        }
      }
      0xE2 => self.ldh(self.reg.get_8bit(&RegCode::C) as usize, RegCode::A),
      0xEF => self.ldh(RegCode::A, self.reg.get_8bit(&RegCode::C) as usize),

      0xCD | 0xCC | 0xDC | 0xC4 | 0xD4 => {
        let addr = self.read_next_addr();
        match opcode {
          0xCD => self.call(true,addr),
          0xCC => self.call(self.reg.check_flag(&Flag::Zero), addr),
          0xDC => self.call(self.reg.check_flag(&Flag::CarryFlag), addr),
          0xC4 => self.call(!self.reg.check_flag(&Flag::Zero), addr),
          0xD4 => self.call(!self.reg.check_flag(&Flag::CarryFlag), addr),
          _ => (),
        }
      }

      0xC5 => self.push(RegCode::BC),
      0xD5 => self.push(RegCode::DE),
      0xE5 => self.push(RegCode::HL),
      0xF5 => self.push(RegCode::AF),

      0xC1 => self.pop(RegCode::BC),
      0xD1 => self.pop(RegCode::BC),
      0xE1 => self.pop(RegCode::BC),
      0xF1 => self.pop(RegCode::BC),

      0xCB => {

      }
      // 0xCB => self.prefixed_opcode(bus),
      _ => ()
    };
  }
}
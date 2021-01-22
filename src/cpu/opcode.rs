use crate::{reg::RegCode, reg::{Flag}};
use crate::cpu::decoder::{OpcodeDecoder,BitMasks};
use crate::debug::logger::LogEvents;

pub trait Load<D,S> {
  fn ld(&mut self, dest: D, source: S);
}

pub trait LoadDec<D,S> {
  fn ldd(&mut self, dest: D, source: S);
}

pub trait LoadInc<D,S> {
  fn ldi(&mut self, dest: D, source: S);
}

pub trait LoadHigh<D,S> {
  fn ldh(&mut self, dest: D, source: S);
}

pub trait IncDec<S> {
  fn inc(&mut self, source: S);
  fn dec(&mut self, source: S);
}

pub trait Xor<S> {
  fn xor(&mut self, source: S);
}

pub trait Compare<S> {
  fn cp(&mut self, source: S);
}

pub trait Rotate {
  fn rl(&mut self, r: RegCode);
  fn rlc(&mut self, r: RegCode);
}

pub trait Jump {
  fn jp(&mut self, condition: bool, next: usize);
  fn jr(&mut self, condition: bool, next: u8);
}

pub trait Stack {
  fn call(&mut self, condition: bool, next: usize);
  fn ret(&mut self);
  fn push(&mut self, reg: RegCode);
  fn pop(&mut self, reg: RegCode);
}

pub trait BitCheck {
  fn check_bit(&mut self, reg: RegCode, mask: u8);
}

 /********************************************************************
 * OPCODE Implementation
 *********************************************************************/

impl Load<RegCode, u8> for OpcodeDecoder<'_> {
  fn ld(&mut self, code: RegCode, byte: u8) {
    self.reg.set_8bit(&code, byte);
  }
}

impl Load<RegCode, RegCode> for OpcodeDecoder<'_> {
  fn ld(&mut self, code: RegCode, source: RegCode) {
    self.reg.set_8bit(&code, self.reg.get_8bit(&source));
  }
}

impl Load<RegCode, (u8,u8)> for OpcodeDecoder<'_> {
  fn ld(&mut self, code: RegCode, source: (u8, u8)) {
    self.reg.set_16bit(&code, source.0, source.1);
  }
}

impl Load<RegCode, &(u8,u8)> for OpcodeDecoder<'_> {
  fn ld(&mut self, code: RegCode, source: &(u8, u8)) {
    self.reg.set_16bit(&code, source.0, source.1);
  }
}

impl Load<usize, RegCode> for OpcodeDecoder<'_> {
  fn ld(&mut self, addr: usize, source: RegCode) {
    self.bus.save(addr, self.reg.get_8bit(&source));
  }
}

impl Load<usize, u8> for OpcodeDecoder<'_> {
  fn ld(&mut self, addr: usize, byte: u8) {
    self.bus.save(addr, byte);
  }
}

impl Load<RegCode, usize> for OpcodeDecoder<'_> {
  fn ld(&mut self, dest: RegCode, addr: usize) {
    let source = self.bus.fetch(addr);
    return self.ld(dest, source);
  }
}

impl LoadDec<usize, RegCode> for OpcodeDecoder<'_> {
  fn ldd(&mut self, dest: usize, source: RegCode) {
    self.ld(dest, source);
    self.reg.dec_hl();
  }
}

impl LoadDec<RegCode, usize> for OpcodeDecoder<'_> {
  fn ldd(&mut self, dest: RegCode, source: usize) {
    self.ld(dest, source);
    self.reg.dec_hl();
  }
}

impl LoadInc<usize, RegCode> for OpcodeDecoder<'_> {
  fn ldi(&mut self, dest: usize, source: RegCode) {
    self.ld(dest, source);
    self.reg.inc_hl();
  }
}

impl LoadInc<RegCode, usize> for OpcodeDecoder<'_> {
  fn ldi(&mut self, dest: RegCode, source: usize) {
    self.ld(dest, source);
    self.reg.inc_hl();
  }
}

impl LoadHigh<RegCode,usize> for OpcodeDecoder<'_> {
  fn ldh(&mut self, code: RegCode, delta: usize) {
    self.reg.set_8bit(&code, self.bus.fetch(0xFF00 + delta));
  }
}

impl LoadHigh<usize,RegCode> for OpcodeDecoder<'_> {
  fn ldh(&mut self, delta: usize, code: RegCode) {
    let byte = self.reg.get_8bit(&code);
    match self.bus.save(0xFF00 + delta as usize, byte) {
      Ok(_) => (),
      Err(_) => panic!("failed")
    }
  }
}

impl Xor<RegCode> for OpcodeDecoder<'_> {
  fn xor(&mut self, code: RegCode) {
    let mut v = self.reg.get_8bit(&code);
    v = self.reg.a ^ v;
    self.reg.set_flag(&Flag::Zero, v == 0);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, false);
    self.reg.set_flag(&Flag::AddSubBCD, false);
    self.reg.set_8bit(&RegCode::A, v);
  }
}

impl Xor<usize> for OpcodeDecoder<'_> {
  fn xor(&mut self, addr: usize) {
    let mut v = self.bus.fetch(addr);
    v = self.reg.a ^ v;
    self.reg.set_flag(&Flag::Zero, v == 0);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, false);
    self.reg.set_flag(&Flag::AddSubBCD, false);
    self.reg.set_8bit(&RegCode::A, v);
  }
}

impl Compare<u8> for OpcodeDecoder<'_> {
  fn cp(&mut self, data: u8) {
    let result = self.reg.a.wrapping_sub(data);
    self.reg.set_flag(&Flag::Zero, result == 0);
    // @TODO: Implement half carry correctly
    self.reg.set_flag(&Flag::AddSubBCD, true);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, false);
    self.reg.set_flag(&Flag::CarryFlag, data > self.reg.a);
  }
}

impl IncDec<RegCode> for OpcodeDecoder<'_> {
  fn inc(&mut self, source: RegCode) {
    match source {
      RegCode::HL => self.reg.inc_hl(),
      _ => self.reg.inc_8bit(&source)
    }
  }

  fn dec(&mut self, source: RegCode) {
    match source {
      RegCode::HL => self.reg.dec_hl(),
      _ => self.reg.dec_8bit(&source)
    }
  }
}

impl IncDec<usize> for OpcodeDecoder<'_> {
  fn inc(&mut self, pointer: usize) {
    let byte = self.bus.fetch(pointer).wrapping_add(1);
    self.reg.set_flag(&Flag::Zero, byte == 0);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, (byte & 0xF) == 0);
    self.reg.set_flag(&Flag::AddSubBCD, false);
    match self.bus.save(pointer, byte) {
        Ok(_) => (),
        Err(_) => panic!("failed")
    }
  }

  fn dec(&mut self, pointer: usize) {
    let byte = self.bus.fetch(pointer).wrapping_sub(1);
    self.reg.set_flag(&Flag::Zero, byte == 0);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, (byte & 0xF) == 0);
    self.reg.set_flag(&Flag::AddSubBCD, false);
    match self.bus.save(pointer, byte) {
        Ok(_) => (),
        Err(_) => panic!("failed")
    }
  }
}

impl Rotate for OpcodeDecoder<'_> {
  fn rlc(&mut self, r: RegCode) {
    let c = self.reg.get_8bit(&r);
    let mc = (c & (BitMasks::Bit7) as u8) >> 7;
    let new_value = (self.reg.get_8bit(&r) << 1) | mc;
    
    self.reg.set_flag(&Flag::CarryFlag, mc != 0);
    self.reg.set_flag(&Flag::Zero, new_value == 0);
    self.reg.set_flag(&Flag::AddSubBCD, false);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, false);

    self.reg.set_8bit(&r, new_value);
  }

  fn rl(&mut self, r: RegCode) {
    let c = self.reg.get_8bit(&r);
    let mc = (c & (BitMasks::Bit7) as u8) >> 7;
    let old_carry: u8 = match self.reg.check_flag(&Flag::CarryFlag) {
      true => 1,
      false => 0,
    };

    let new_value = (self.reg.get_8bit(&r) << 1) | old_carry;
    
    self.reg.set_flag(&Flag::CarryFlag, mc != 0);
    self.reg.set_flag(&Flag::Zero, new_value == 0);
    self.reg.set_flag(&Flag::AddSubBCD, false);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, false);

    self.reg.set_8bit(&r, new_value);
  }
}

impl Jump for OpcodeDecoder<'_> {
  fn jp(&mut self, condition: bool, pointer: usize) {
    if condition {
      self.message_buffer.push((LogEvents::Register, format!("[JP] Setting PC to {:#06x}", pointer)));
      self.reg.set_pc(pointer);
    }
  }

  fn jr(&mut self, condition: bool, byte: u8) {
    let current_pc = self.reg.pc as u16;
    let next = (current_pc as i32 + (byte as i8) as i32) as usize;
        if condition {
      self.reg.set_pc(next);
    }
  }
}

impl Stack for OpcodeDecoder<'_> {
  fn call(&mut self, condition: bool, next: usize) {
    if condition {
      self.reg.sp = self.reg.sp - 2;
      self.bus.save(self.reg.sp as usize, (self.reg.pc & 0x00FF) as u8);
      self.bus.save((self.reg.sp+1) as usize, ((self.reg.pc & 0xFF00) >> 8) as u8);
      self.jp(true, next);
    }
  }

  fn ret(&mut self) {
    let left_byte = self.bus.fetch((self.reg.sp+1) as usize);
    let right_byte = self.bus.fetch(self.reg.sp as usize);
    self.reg.set_16bit(&RegCode::PC, left_byte, right_byte);
  }

  fn push(&mut self, reg: RegCode) {
    self.reg.sp -= 2;
    let data = self.reg.get_16bit(&reg);
    self.bus.save(self.reg.sp as usize, ((data & 0xFF00) >> 8) as u8);
    self.bus.save((self.reg.sp + 1) as usize, (data & 0x00FF) as u8 );
    // let right_byte = self.bus.fetch((self.reg.sp+1) as usize);
    // let left_byte = self.bus.fetch(self.reg.sp as usize);
    // self.reg.set_16bit(&reg, left_byte, right_byte);
  }

  fn pop(&mut self, reg: RegCode) {
    let right_byte = self.bus.fetch((self.reg.sp+1) as usize);
    let left_byte = self.bus.fetch(self.reg.sp as usize);
    self.reg.set_16bit(&reg, left_byte, right_byte);
    self.reg.sp += 2;
  }
}

impl BitCheck for OpcodeDecoder<'_> {
  fn check_bit(&mut self, code: RegCode, mask: u8) {
    self.reg.set_flag(&Flag::Zero, self.reg.get_8bit(&code) & mask != mask);
    self.reg.set_flag(&Flag::AddSubBCD, true);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, false);
  }
}

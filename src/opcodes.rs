use crate::mmu::VirtualMemory;
use crate::cpu::CPU;
use crate::debug::logger::LogEvents;
use crate::reg::{Flag,RegCode};

pub enum OpCodeSuccess{
  PostIncrement,
  NoPostIncrement,
}

enum BitMasks {
  Bit1 = 0b1,
  Bit2 = 0b10,
  Bit3 = 0b100,
  Bit4 = 0b1000,
  Bit5 = 0b10000,
  Bit6 = 0b100000,
  Bit7 = 0b1000000,
  Bit8 = 0b10000000,
}

pub trait OpCodes {
  fn run_opcode(&mut self, opcode: u8, bus: &mut VirtualMemory);

  fn ld_reg_u8(&mut self, bus: &mut VirtualMemory, dest: &RegCode, value: u8);
  fn ld_reg_u16_addr(&mut self, bus: &mut VirtualMemory, dest: &RegCode, addr: u16);
  fn ldi_reg_u16_addr(&mut self, bus: &mut VirtualMemory, dest: &RegCode, addr: u16);
  fn ldd_reg_u16_addr(&mut self, bus: &mut VirtualMemory, dest: &RegCode, addr: u16);
  fn ld_reg_u16_direct(&mut self, bus: &mut VirtualMemory, dest: &RegCode);
  fn ld_reg_u8_direct(&mut self, bus: &mut VirtualMemory, dest: &RegCode);

  fn ld_mem_u8(&mut self, bus: &mut VirtualMemory, addr_dest: u16, value: u8);
  fn ld_mem_d8(&mut self, bus: &mut VirtualMemory, addr_dest: u16);
  fn ldi_mem_u8(&mut self, bus: &mut VirtualMemory, addr_dest: u16, value: u8);
  fn ldd_mem_u8(&mut self, bus: &mut VirtualMemory, addr_dest: u16, value: u8);

  fn ldh_a_d8(&mut self, bus: &mut VirtualMemory);
  fn ldh_a_regc(&mut self, bus: &mut VirtualMemory);
  fn ldh_a8_a(&mut self, bus: &mut VirtualMemory);
  fn ldh_regc_a(&mut self, bus: &mut VirtualMemory);

  fn inc_reg(&mut self, bus: &mut VirtualMemory, dest: &RegCode);
  fn inc_mem_u16(&mut self, bus: &mut VirtualMemory, addr: u16);
  fn dec_reg(&mut self, bus: &mut VirtualMemory, dest: &RegCode);
  fn dec_mem_u16(&mut self, bus: &mut VirtualMemory, addr: u16);

  fn jr_cond(&mut self, bus: &mut VirtualMemory, cond: bool);
  fn jp_cond(&mut self, bus: &mut VirtualMemory, cond: bool);

  fn pop(&mut self, bus: &mut VirtualMemory, r: &RegCode);
  fn push(&mut self, bus: &mut VirtualMemory, r: &RegCode);

  fn xaf_xor_a(&mut self, bus: &mut VirtualMemory);
  fn x82_add_a_d(&mut self, bus: &mut VirtualMemory);

  fn call(&mut self, bus: &mut VirtualMemory, cond: bool); 
  fn prefixed_opcode(&mut self, bus: &mut VirtualMemory);

  fn check_bit(&mut self, bus: &mut VirtualMemory, r: &RegCode, mask: u8);

  fn rl(&mut self, bus: &mut VirtualMemory, r: &RegCode);
  fn rlc(&mut self, bus: &mut VirtualMemory, r: &RegCode);
}

impl OpCodes for CPU {
  fn run_opcode(&mut self, byte: u8, bus: &mut VirtualMemory) { 
    match byte {
      0x00 => (),
      0x01 => self.ld_reg_u16_direct(bus, &RegCode::BC),
      0x11 => self.ld_reg_u16_direct(bus, &RegCode::DE),
      0x21 => self.ld_reg_u16_direct(bus, &RegCode::HL),
      0x31 => self.ld_reg_u16_direct(bus, &RegCode::SP),

      0x02 => self.ld_mem_u8(bus, self.reg.get_16bit(&RegCode::BC), self.reg.a),
      0x12 => self.ld_mem_u8(bus, self.reg.get_16bit(&RegCode::DE), self.reg.a),
      0x22 => self.ldi_mem_u8(bus, self.reg.get_16bit(&RegCode::HL), self.reg.a),
      0x32 => self.ldd_mem_u8(bus, self.reg.get_16bit(&RegCode::HL), self.reg.a),

      0x04 => self.inc_reg(bus, &RegCode::B),
      0x14 => self.inc_reg(bus, &RegCode::D),
      0x24 => self.inc_reg(bus, &RegCode::H),
      0x34 => self.inc_mem_u16(bus, self.reg.get_16bit(&RegCode::HL)),

      0x05 => self.dec_reg(bus, &RegCode::B),
      0x15 => self.dec_reg(bus, &RegCode::D),
      0x25 => self.dec_reg(bus, &RegCode::H),
      0x35 => self.dec_mem_u16(bus, self.reg.get_16bit(&RegCode::HL)),

      0x06 => self.ld_reg_u8_direct(bus, &RegCode::B),
      0x16 => self.ld_reg_u8_direct(bus, &RegCode::D),
      0x26 => self.ld_reg_u8_direct(bus, &RegCode::H),
      0x36 => self.ld_mem_d8(bus, self.reg.get_16bit(&RegCode::HL)),

      0x07 => self.rlc(bus, &RegCode::A),
      0x17 => self.rl(bus, &RegCode::A),

      0x0a => self.ld_reg_u16_addr(bus, &RegCode::A, self.reg.get_16bit(&RegCode::BC)),
      0x1a => self.ld_reg_u16_addr(bus, &RegCode::A, self.reg.get_16bit(&RegCode::DE)),
      0x2a => self.ldi_reg_u16_addr(bus, &RegCode::A, self.reg.get_16bit(&RegCode::HL)),
      0x3a => self.ldd_reg_u16_addr(bus, &RegCode::A, self.reg.get_16bit(&RegCode::HL)),

      0x0c => self.inc_reg(bus, &RegCode::C),
      0x1c => self.inc_reg(bus, &RegCode::E),
      0x2c => self.inc_reg(bus, &RegCode::L),
      0x3c => self.inc_reg(bus, &RegCode::A),

      0x0e => self.ld_reg_u8_direct(bus, &RegCode::C),
      0x1e => self.ld_reg_u8_direct(bus, &RegCode::E),
      0x2e => self.ld_reg_u8_direct(bus, &RegCode::L),
      0x3e => self.ld_reg_u8_direct(bus, &RegCode::A),

      0x40 => self.ld_reg_u8(bus, &RegCode::B, self.reg.b),
      0x41 => self.ld_reg_u8(bus, &RegCode::B, self.reg.c),
      0x42 => self.ld_reg_u8(bus, &RegCode::B, self.reg.d),
      0x43 => self.ld_reg_u8(bus, &RegCode::B, self.reg.e),
      0x44 => self.ld_reg_u8(bus, &RegCode::B, self.reg.h),
      0x45 => self.ld_reg_u8(bus, &RegCode::B, self.reg.l),
      0x46 => self.ld_reg_u16_addr(bus, &RegCode::B, self.reg.get_16bit(&RegCode::HL)),
      0x47 => self.ld_reg_u8(bus, &RegCode::B, self.reg.a),
      0x48 => self.ld_reg_u8(bus, &RegCode::C, self.reg.b),
      0x49 => self.ld_reg_u8(bus, &RegCode::C, self.reg.c),
      0x4A => self.ld_reg_u8(bus, &RegCode::C, self.reg.d),
      0x4B => self.ld_reg_u8(bus, &RegCode::C, self.reg.e),
      0x4C => self.ld_reg_u8(bus, &RegCode::C, self.reg.h),
      0x4D => self.ld_reg_u8(bus, &RegCode::C, self.reg.l),
      0x4E => self.ld_reg_u16_addr(bus, &RegCode::C, self.reg.get_16bit(&RegCode::HL)),
      0x4F => self.ld_reg_u8(bus, &RegCode::C, self.reg.a),

      0x50 => self.ld_reg_u8(bus, &RegCode::D, self.reg.b),
      0x51 => self.ld_reg_u8(bus, &RegCode::D, self.reg.c),
      0x52 => self.ld_reg_u8(bus, &RegCode::D, self.reg.d),
      0x53 => self.ld_reg_u8(bus, &RegCode::D, self.reg.e),
      0x54 => self.ld_reg_u8(bus, &RegCode::D, self.reg.h),
      0x55 => self.ld_reg_u8(bus, &RegCode::D, self.reg.l),
      0x56 => self.ld_reg_u16_addr(bus, &RegCode::D, self.reg.get_16bit(&RegCode::HL)),
      0x57 => self.ld_reg_u8(bus, &RegCode::D, self.reg.a),
      0x58 => self.ld_reg_u8(bus, &RegCode::E, self.reg.b),
      0x59 => self.ld_reg_u8(bus, &RegCode::E, self.reg.c),
      0x5A => self.ld_reg_u8(bus, &RegCode::E, self.reg.d),
      0x5B => self.ld_reg_u8(bus, &RegCode::E, self.reg.e),
      0x5C => self.ld_reg_u8(bus, &RegCode::E, self.reg.h),
      0x5D => self.ld_reg_u8(bus, &RegCode::E, self.reg.l),
      0x5E => self.ld_reg_u16_addr(bus, &RegCode::E, self.reg.get_16bit(&RegCode::HL)),
      0x5F => self.ld_reg_u8(bus, &RegCode::E, self.reg.a),

      0x60 => self.ld_reg_u8(bus, &RegCode::H, self.reg.b),
      0x61 => self.ld_reg_u8(bus, &RegCode::H, self.reg.c),
      0x62 => self.ld_reg_u8(bus, &RegCode::H, self.reg.d),
      0x63 => self.ld_reg_u8(bus, &RegCode::H, self.reg.e),
      0x64 => self.ld_reg_u8(bus, &RegCode::H, self.reg.h),
      0x65 => self.ld_reg_u8(bus, &RegCode::H, self.reg.l),
      0x66 => self.ld_reg_u16_addr(bus, &RegCode::H, self.reg.get_16bit(&RegCode::HL)),
      0x67 => self.ld_reg_u8(bus, &RegCode::H, self.reg.a),
      0x68 => self.ld_reg_u8(bus, &RegCode::L, self.reg.b),
      0x69 => self.ld_reg_u8(bus, &RegCode::L, self.reg.c),
      0x6A => self.ld_reg_u8(bus, &RegCode::L, self.reg.d),
      0x6B => self.ld_reg_u8(bus, &RegCode::L, self.reg.e),
      0x6C => self.ld_reg_u8(bus, &RegCode::L, self.reg.h),
      0x6D => self.ld_reg_u8(bus, &RegCode::L, self.reg.l),
      0x6E => self.ld_reg_u16_addr(bus, &RegCode::L, self.reg.get_16bit(&RegCode::HL)),
      0x6F => self.ld_reg_u8(bus, &RegCode::L, self.reg.a),

      0x70 => self.ld_mem_u8(bus, self.reg.get_16bit(&RegCode::HL), self.reg.b),
      0x71 => self.ld_mem_u8(bus, self.reg.get_16bit(&RegCode::HL), self.reg.c),
      0x72 => self.ld_mem_u8(bus, self.reg.get_16bit(&RegCode::HL), self.reg.d),
      0x73 => self.ld_mem_u8(bus, self.reg.get_16bit(&RegCode::HL), self.reg.e),
      0x74 => self.ld_mem_u8(bus, self.reg.get_16bit(&RegCode::HL), self.reg.h),
      0x75 => self.ld_mem_u8(bus, self.reg.get_16bit(&RegCode::HL), self.reg.l),
      0x77 => self.ld_mem_u8(bus, self.reg.get_16bit(&RegCode::HL), self.reg.a),

      0x78 => self.ld_reg_u8(bus, &RegCode::A, self.reg.b),
      0x79 => self.ld_reg_u8(bus, &RegCode::A, self.reg.c),
      0x7A => self.ld_reg_u8(bus, &RegCode::A, self.reg.d),
      0x7B => self.ld_reg_u8(bus, &RegCode::A, self.reg.e),
      0x7C => self.ld_reg_u8(bus, &RegCode::A, self.reg.h),
      0x7D => self.ld_reg_u8(bus, &RegCode::A, self.reg.l),
      0x7E => self.ld_reg_u16_addr(bus, &RegCode::A, self.reg.get_16bit(&RegCode::HL)),
      0x7F => self.ld_reg_u8(bus, &RegCode::A, self.reg.a),

      0x20 => self.jr_cond(bus, !self.reg.check_flag(&Flag::Zero)),
      0x30 => self.jr_cond(bus, !self.reg.check_flag(&Flag::CarryFlag)),
      0x81 => self.jr_cond(bus, true),
      0x82 => self.jr_cond(bus, self.reg.check_flag(&Flag::Zero)),
      0x83 => self.jr_cond(bus, self.reg.check_flag(&Flag::CarryFlag)),

      0xC2 => self.jp_cond(bus, !self.reg.check_flag(&Flag::Zero)),
      0xD2 => self.jp_cond(bus, !self.reg.check_flag(&Flag::CarryFlag)),
      0xC3 => self.jp_cond(bus, true),
      0xCA => self.jp_cond(bus, self.reg.check_flag(&Flag::Zero)),
      0xDA => self.jp_cond(bus, self.reg.check_flag(&Flag::CarryFlag)),

      0xAF => self.xaf_xor_a(bus, ),

      0xE0 => self.ldh_a8_a(bus, ),
      0xF0 => self.ldh_a_d8(bus, ),
      0xE2 => self.ldh_regc_a(bus, ),
      0xEF => self.ldh_a8_a(bus, ),

      0xCD => self.call(bus, true),
      0xCC => self.call(bus, self.reg.check_flag(&Flag::Zero)),
      0xDC => self.call(bus, self.reg.check_flag(&Flag::CarryFlag)),
      0xC4 => self.call(bus, !self.reg.check_flag(&Flag::Zero)),
      0xD4 => self.call(bus, !self.reg.check_flag(&Flag::CarryFlag)),

      0xC5 => self.push(bus, &RegCode::BC),
      0xD5 => self.push(bus, &RegCode::DE),
      0xE5 => self.push(bus, &RegCode::HL),
      0xF5 => self.push(bus, &RegCode::AF),

      0xC1 => self.pop(bus, &RegCode::BC),
      0xD1 => self.pop(bus, &RegCode::BC),
      0xE1 => self.pop(bus, &RegCode::BC),
      0xF1 => self.pop(bus, &RegCode::BC),

      0xCB => self.prefixed_opcode(bus),
      _ => ()
    };
  }

  fn prefixed_opcode(&mut self, bus: &mut VirtualMemory) {
    let next_byte = self.fetch_post_increment(bus);
    match next_byte {
      0x11 => self.rl(bus, &RegCode::C),
      0x7C => self.check_bit(bus, &RegCode::H, BitMasks::Bit7 as u8),
      _ => (),
    }
  }

  fn inc_reg(&mut self, bus: &mut VirtualMemory, dest: &RegCode) {
    self.reg.inc_8bit(dest);
    self.reg.set_flag(&Flag::Zero, self.reg.get_8bit(dest) == 0);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, (self.reg.get_8bit(dest) & 0xF) == 0);
    self.reg.set_flag(&Flag::AddSubBCD, false);
  }

  fn inc_mem_u16(&mut self, bus: &mut VirtualMemory, addr: u16) {
    let m = bus.fetch(addr as usize) + 1;
    self.reg.set_flag(&Flag::Zero, m == 0);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, (m & 0xF) == 0);
    self.reg.set_flag(&Flag::AddSubBCD, false);
    match bus.save(addr as usize, m) {
        Ok(_) => (),
        Err(_) => panic!("failed")
    }
  }

  fn dec_reg(&mut self, bus: &mut VirtualMemory, dest: &RegCode) {
    self.reg.dec_8bit(dest);
    self.reg.set_flag(&Flag::Zero, self.reg.get_8bit(dest) == 0);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, (self.reg.get_8bit(dest) & 0xF) == 0);
    self.reg.set_flag(&Flag::AddSubBCD, true);
  }

  fn dec_mem_u16(&mut self, bus: &mut VirtualMemory, addr: u16) {
    let m = bus.fetch(addr as usize) - 1;
    self.reg.set_flag(&Flag::Zero, m == 0);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, (m & 0xF) == 0);
    self.reg.set_flag(&Flag::AddSubBCD, true);
    match bus.save(addr as usize, m) {
        Ok(_) => (),
        Err(_) => panic!("failed")
    }
  }

  fn ld_mem_u8(&mut self, bus: &mut VirtualMemory, addr_dest: u16, value: u8) {
    self.log(LogEvents::MemorySave, format!("[{:#06x}] < {:04x}", addr_dest, value));
    bus.save(addr_dest as usize, value);
  }

  fn ld_mem_d8(&mut self, bus: &mut VirtualMemory, addr_dest: u16) {
    let next = self.fetch_post_increment(bus);
    self.log(LogEvents::MemorySave, format!("[{:#06x}] < {:04x}", addr_dest, next));
    bus.save(addr_dest as usize, next);
  }

  fn ldi_mem_u8(&mut self, bus: &mut VirtualMemory, addr_dest: u16, value: u8) {
    let result = self.ld_mem_u8(bus, addr_dest, value);
    self.reg.inc_hl();
    return result;
  }

  fn ldd_mem_u8(&mut self, bus: &mut VirtualMemory, addr_dest: u16, value: u8) {
    let result = self.ld_mem_u8(bus, addr_dest, value);
    self.reg.dec_hl();
    return result;
  }

  fn ld_reg_u8_direct(&mut self, bus: &mut VirtualMemory, dest: &RegCode) {
    let next = self.fetch_post_increment(bus);
    self.reg.set_8bit(&dest, next);
  }

  fn ld_reg_u16_direct(&mut self, bus: &mut VirtualMemory, dest: &RegCode) {
    let right_byte = self.fetch_post_increment(bus);
    let left_byte = self.fetch_post_increment(bus);
    self.reg.set_16bit(dest, left_byte, right_byte);
  }

  fn ld_reg_u16_addr(&mut self, bus: &mut VirtualMemory, dest: &RegCode, addr: u16) {
    let value = bus.fetch(addr as usize);
    return self.ld_reg_u8(bus, dest, value);
  }

  fn ldi_reg_u16_addr(&mut self, bus: &mut VirtualMemory, dest: &RegCode, addr: u16) {
    let result = self.ld_reg_u16_addr(bus, dest, addr);
    self.reg.inc_hl();
    return result;
  }

  fn ldd_reg_u16_addr(&mut self, bus: &mut VirtualMemory, dest: &RegCode, addr: u16) {
    let result = self.ld_reg_u16_addr(bus, dest, addr);
    self.reg.dec_hl();
    return result;
  }

  fn ld_reg_u8(&mut self, bus: &mut VirtualMemory, dest: &RegCode, value: u8) {
    match dest {
        RegCode::A => { self.reg.a = value },
        RegCode::F => { self.reg.f = value },
        RegCode::B => { self.reg.b = value },
        RegCode::C => { self.reg.c = value },
        RegCode::D => { self.reg.d = value },
        RegCode::E => { self.reg.e = value },
        RegCode::H => { self.reg.h = value },
        RegCode::L => { self.reg.l = value },
        _ => {
          panic!("Cannot set 8bit value to 16bit reg")
        },
    };
  }

  fn ldh_a_d8(&mut self, bus: &mut VirtualMemory) {
    let delta = self.fetch_post_increment(bus);
    self.reg.a = bus.fetch((0xFF00 + (delta as u16)) as usize);
  }

  fn ldh_a_regc(&mut self, bus: &mut VirtualMemory) {
    let delta = self.reg.c;
    self.reg.a = bus.fetch((0xFF00 + (delta as u16)) as usize);
  }

  fn ldh_a8_a(&mut self, bus: &mut VirtualMemory) {
    let delta = self.fetch_post_increment(bus);
    match bus.save((0xFF00 + (delta as u16)) as usize, self.reg.a) {
      Ok(_) => (),
      Err(_) => panic!("failed")
    }
  }

  fn ldh_regc_a(&mut self, bus: &mut VirtualMemory) {
    let delta = self.reg.c;
    match bus.save((0xFF00 + (delta as u16)) as usize, self.reg.a) {
      Ok(_) => (),
      Err(_) => panic!("failed")
    }
  }

  fn xaf_xor_a(&mut self, bus: &mut VirtualMemory) {
    self.reg.a = self.reg.a ^ self.reg.a;
  }

  fn x82_add_a_d(&mut self, bus: &mut VirtualMemory) {
    let partial_result: u16 = self.reg.a as u16 + self.reg.d as u16;
    self.reg.set_flag(&Flag::CarryFlag, partial_result > 0xFF);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, (self.reg.a & 0xF) + (self.reg.d & 0xF) > 0xF);
    
    self.reg.a += self.reg.d;
    self.reg.set_flag(&Flag::Zero, self.reg.a == 0);

  }

  fn jp_cond(&mut self, bus: &mut VirtualMemory, cond: bool) {
    let mut new_pointer: usize = self.fetch_post_increment(bus) as usize;
    new_pointer += (self.fetch_post_increment(bus) as usize) << 8;
    if cond {
      self.set_pc(new_pointer);
    }
  }

  fn jr_cond(&mut self, bus: &mut VirtualMemory, cond: bool) {
    let current_pc = self.reg.pc as u16;
    let data: i8 = self.fetch_post_increment(bus) as i8;
    let next = (current_pc as i32 + data as i32) as usize;

    if cond {
      self.set_pc(next);
    }
  }

  fn pop(&mut self, bus: &mut VirtualMemory, r: &RegCode) {
    let right_byte = bus.fetch(self.reg.sp as usize);
    let left_byte = bus.fetch((self.reg.sp+1) as usize);
    self.reg.set_16bit(r, left_byte, right_byte);
    self.reg.sp += 2;
  }

  fn push(&mut self, bus: &mut VirtualMemory, r: &RegCode) {
    let right_byte = bus.fetch(self.reg.sp as usize);
    let left_byte = bus.fetch((self.reg.sp-1) as usize);
    self.reg.set_16bit(r, left_byte, right_byte);
    self.reg.sp -= 2;
  }

  fn call(&mut self, bus: &mut VirtualMemory, cond: bool) {
    self.reg.sp = self.reg.sp - 2;
    bus.save(self.reg.sp as usize, (self.reg.pc & 0x00FF) as u8);
    bus.save((self.reg.sp+1) as usize, ((self.reg.pc & 0xFF00) >> 8) as u8);

    return self.jp_cond(bus, true);
  }

  fn check_bit(&mut self, bus: &mut VirtualMemory, r: &RegCode, mask: u8) {
    self.reg.set_flag(&Flag::Zero, self.reg.get_8bit(r) & mask != 0);
    self.reg.set_flag(&Flag::AddSubBCD, true);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, false);
  }

  fn rl(&mut self, bus: &mut VirtualMemory, r: &RegCode) {
    let c = self.reg.get_8bit(r);
    let mc = c & (BitMasks::Bit8) as u8;
    let new_value = (self.reg.get_8bit(r) << 1) | mc;
    
    self.reg.set_flag(&Flag::CarryFlag, mc != 0);
    self.reg.set_flag(&Flag::Zero, new_value == 0);
    self.reg.set_flag(&Flag::AddSubBCD, false);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, false);

    self.reg.set_8bit(r, new_value);
  }

  fn rlc(&mut self, bus: &mut VirtualMemory, r: &RegCode) {
    let c = self.reg.get_8bit(r);
    let mc = c & (BitMasks::Bit8) as u8;
    let old_carry: u8 = match self.reg.check_flag(&Flag::CarryFlag) {
      true => 1,
      false => 0,
    };

    let new_value = (self.reg.get_8bit(r) << 1) | old_carry;
    
    self.reg.set_flag(&Flag::CarryFlag, mc != 0);
    self.reg.set_flag(&Flag::Zero, new_value == 0);
    self.reg.set_flag(&Flag::AddSubBCD, false);
    self.reg.set_flag(&Flag::HalfCarryFlagBCD, false);

    self.reg.set_8bit(r, new_value);
  }
}
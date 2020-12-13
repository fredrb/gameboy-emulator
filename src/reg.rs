pub enum Flag {
  Zero,
  AddSubBCD,
  HalfCarryFlagBCD,
  CarryFlag
}

#[derive(Debug)]
pub enum RegCode {
  A,
  F,
  B,
  C,
  D,
  E,
  H,
  L,
  SP,
  PC,
  AF,
  BC,
  DE,
  HL
}
  pub struct Registers {
  pub a: u8,
  pub f: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
  pub sp: u16,
  pub pc: usize
}

impl Registers {
  pub fn new() -> Self {
      Registers {
        // Initial GBC Model => @TODO: Check
        a: 0x11,
        f: 0x80,
        b: 0x00,
        c: 0x00,
        d: 0xFF,
        e: 0x56,
        h: 0x00,
        l: 0x7C,
        sp: 0xFFFE,
        pc: 0x0000
    }
  }

  pub fn print_registers(&self) -> Vec<String> {
    vec!(
      format!("AF {:#06x}", self.get_16bit(&RegCode::AF)),
      format!("BC {:#06x}", self.get_16bit(&RegCode::BC)),
      format!("DE {:#06x}", self.get_16bit(&RegCode::DE)),
      format!("HL {:#06x}", self.get_16bit(&RegCode::HL)),
      format!("SP {:#06x}", self.get_16bit(&RegCode::SP)),
      format!("PC {:#06x}", self.get_16bit(&RegCode::PC))
    )
  }

  pub fn _8bit_to_16bit(left: u8, right: u8) -> u16 {
    let res = (left as u16) << 8;
    return res + (right as u16);
  }

  fn get_mask(&self, flag: &Flag) -> u8 {
    return match flag {
      Flag::AddSubBCD => 0b01000000 as u8,
      Flag::CarryFlag => 0b00010000 as u8,
      Flag::HalfCarryFlagBCD => 0b00100000 as u8,
      Flag::Zero => 0b10000000 as u8
    };
  }

  pub fn inc_8bit(&mut self, code: &RegCode) {
    let byte = self.get_8bit(code).wrapping_add(1);
    self.set_flag(&Flag::HalfCarryFlagBCD, byte & 0xF == 0);
    self.set_flag(&Flag::Zero, byte == 0);
    self.set_flag(&Flag::AddSubBCD, false);
    self.set_8bit(code, byte);
  }

  pub fn dec_8bit(&mut self, code: &RegCode) {
    let byte = self.get_8bit(code).wrapping_sub(1);
    self.set_flag(&Flag::HalfCarryFlagBCD, byte & 0xF == 0);
    self.set_flag(&Flag::Zero, byte == 0);
    self.set_flag(&Flag::AddSubBCD, true);
    self.set_8bit(code, byte)
  }

  pub fn inc_pc(&mut self) {
    let current_value = self.get_16bit(&RegCode::PC).wrapping_add(1);
    self.pc = current_value as usize;
  }

  pub fn inc_hl(&mut self) {
    let current_value = self.get_16bit(&RegCode::HL).wrapping_add(1);
    self.set_flag(&Flag::HalfCarryFlagBCD, current_value & 0xFF == 0);
    self.set_flag(&Flag::Zero, current_value == 0);
    self.set_flag(&Flag::AddSubBCD, false);
    self.set_16bit(&RegCode::HL, 
      (current_value & 0xF0) as u8,
       (current_value & 0x0F) as u8);
  }

  pub fn dec_hl(&mut self) {
    let current_value = self.get_16bit(&RegCode::HL);
    let subbed_value = current_value.wrapping_sub(1);
    self.set_flag(&Flag::HalfCarryFlagBCD, subbed_value & 0xFF == 0);
    self.set_flag(&Flag::Zero, subbed_value == 0);
    self.set_flag(&Flag::AddSubBCD, true);
    self.set_16bit(&RegCode::HL, 
      ((subbed_value & 0xFF00) >> 8) as u8,
       (subbed_value & 0x00FF) as u8);
  }

  pub fn set_8bit(&mut self, code: &RegCode, byte: u8) {
    match code {
        RegCode::A => {self.a = byte}
        RegCode::F => {self.f = byte}
        RegCode::B => {self.b = byte}
        RegCode::C => {self.c = byte}
        RegCode::D => {self.d = byte}
        RegCode::E => {self.e = byte}
        RegCode::H => {self.h = byte}
        RegCode::L => {self.l = byte}
        _ => panic!("This opeartion only supports 8but registers")
    }
  }

  pub fn set_16bit(&mut self, code: &RegCode, left_byte: u8, right_byte: u8) {
    match code {
        RegCode::AF => {self.a = left_byte; self.f = right_byte}
        RegCode::BC => {self.b = left_byte; self.c = right_byte}
        RegCode::DE => {self.d = left_byte; self.e = right_byte}
        RegCode::HL => {self.h = left_byte; self.l = right_byte}
        RegCode::SP => {self.sp = Registers::_8bit_to_16bit(left_byte, right_byte)}
        _ => panic!("This operation only supports 16bit regiters")
    }
  }

  pub fn set_pc(&mut self, pointer: usize) {
    self.pc = pointer;
  }

  pub fn get_16bit(&self, code: &RegCode) -> u16 {
    let tuple = match code {
        RegCode::AF => (self.a, self.f),
        RegCode::BC => (self.b, self.c),
        RegCode::DE => (self.d, self.e),
        RegCode::HL => (self.h, self.l),
        RegCode::SP => return self.sp,
        RegCode::PC => return self.pc as u16,
        // @TODO: Change RegisterCode to have both 16bit and 8bit enums
        _ => panic!("Cannot use 8bit register here")
    };
    return Registers::_8bit_to_16bit(tuple.0, tuple.1);
  }

  pub fn get_8bit(&self, code: &RegCode) -> u8 {
    return match code {
        RegCode::A => self.a,
        RegCode::F => self.f,
        RegCode::B => self.b,
        RegCode::C => self.c,
        RegCode::D => self.d,
        RegCode::E => self.e,
        RegCode::H => self.h,
        RegCode::L => self.l,
        _ => panic!("Cannot use 16bit register here")
    }
  }

  pub fn check_flag(&self, flag: &Flag) -> bool {
    return self.f & self.get_mask(flag) == self.get_mask(flag);
  }

  pub fn set_flag(&mut self, flag: &Flag, set: bool) {
    let mask = self.get_mask(flag);
    self.f = match set {
      true => self.f | mask,
      false => self.f & !mask
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_zero_flag_and_rest_null() {
      let reg = Registers::new();
      assert!(reg.check_flag(&Flag::Zero));
      assert!(!reg.check_flag(&Flag::AddSubBCD));
      assert!(!reg.check_flag(&Flag::HalfCarryFlagBCD));
      assert!(!reg.check_flag(&Flag::CarryFlag));
    }
}
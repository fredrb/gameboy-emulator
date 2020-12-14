use console::Style;
use console::style;
use console::Term;
use crate::reg::Registers;

pub struct Terminal {
  line_width: u16
}

pub enum MessageType {
  Normal,
  Good,
  Bad,
  Important
}

impl Terminal {

  pub fn new(width: u16) -> Self {
    Terminal {
      line_width: width
    }
  }

  fn get_style(&self, value: u8, index: usize, primary_index: usize, next_index: usize) -> console::StyledObject<u8> {
    let secondary = primary_index + next_index;
    match index {
      x if x == primary_index => style(value).bold().green(),
      x if x > primary_index && x <= secondary => style(value).bold(),
      _ => style(value).white()
    }
  }

  pub fn print_message(&self, message_type: MessageType, message: &str) {
    let value = match message_type {
      MessageType::Normal => style(message),
      MessageType::Good => style(message).green().bold(),
      MessageType::Bad => style(message).red().bold(),
      MessageType::Important => style(message).bold(),
    };
    println!("{}", value);
  }

  pub fn print_hextable(&self, slice: &[u8], primary_index: usize, next_index: usize) {
    let mut column = 0;
    let mut i = 0;
    for hex in slice {
      if column % self.line_width == 0 {
        print!("\n");
        column = 0;
      }
      print!("{:#04x} ", self.get_style(hex.clone(), i as usize, primary_index, next_index));
      column += 1;
      i += 1;
    }
    println!("\n{}: {:#06x}", style("PC").bold(), primary_index);
  }

  pub fn print_memory(&self, start: usize, slice: &[u8], middle: usize) {
    for addr in 0..slice.len() {
      if addr == middle {
        println!("[{:#06x}]: {:#04x}", style(start + addr).bold(), style(slice[addr]).green().bold());
      } else {
        println!("[{:#06x}]: {:#04x}", style(start + addr).bold(), slice[addr]);
      }
    }
  }

  pub fn print_registers(&self, registers: &Registers) {
    println!("{}: {:#04x} {:#04x} ({:08b})", style("AF").bold(), registers.a, registers.f, registers.f);
    println!("{}: {:#04x} {:#04x}", style("BC").bold(), registers.b, registers.c);
    println!("{}: {:#04x} {:#04x}", style("HL").bold(), registers.h, registers.l);
    println!("{}: {:#06x}", style("SP").bold(), registers.sp);
    println!("{}: {:#06x}", style("PC").bold(), registers.pc);
  }

  pub fn print_breakpoints(&self, breakpoints: &Vec<u16>) {
    for b in 0..breakpoints.len() {
      println!("{}: [{:#06x}]", b, breakpoints[b]);
    }
  }
}
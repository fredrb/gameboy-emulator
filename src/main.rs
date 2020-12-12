mod external;
mod cpu;
mod mmu;
mod opcodes;
mod reg;
mod gameboy;
mod gb_config;
mod debug;

use std::env;
use external::cartridge::{self, Cartridge};
use crate::debug::logger::LogEvents;

fn main() {
  let args: Vec<String> = env::args().collect();
  let cartridge_filename = &args[1];

  let file_path: String = String::from(cartridge_filename);
  let cartridge = cartridge::Cartridge::from_file(&file_path);

  let cfg = gb_config::gb_config::new(&String::from("Settings"));

  let mut logger = debug::logger::Logger::new(cfg.clone());
  let client = logger.make_client();

  let mut gb = gameboy::Gameboy::new(cfg, client);
  
  let logger_thread = std::thread::spawn(move || {
    loop {
      match logger.poll_message() {
        Ok(_) => (),
        Err(why) => panic!("Polling message failed {}", why)
      }
    }
  });

  gb.load_cartridge(cartridge);
  gb.start();
  logger_thread.join().unwrap();
}

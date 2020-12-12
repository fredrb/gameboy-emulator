use std::io::Read;
use std::fs::File;

pub fn load_boot_rom(path: &str) -> Vec<u8> {
  let mut file = match File::open(path) {
    Err(why) => panic!("Couldn't open {}: {}", path, why),
    Ok(f) => f
  };

  let mut buf: Vec<u8> = Vec::new();
  match file.read_to_end(&mut buf) {
      Err(why) => panic!("Couldn't read byte stream: {}", why),
      Ok(size) => {
          println!("Read {} bytes of data", size);
          return buf;
      }
  }
}
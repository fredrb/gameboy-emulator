
#[derive(Clone)]
pub struct gb_config {
  pub boot_rom_enabled: bool,
  pub boot_rom_path: String
}

impl gb_config {
  pub fn new(path: &str) -> Self {
    let mut settings = config::Config::default();
    return match settings.merge(config::File::with_name(path)) {
      Ok(c) => gb_config {
          boot_rom_enabled: c.get_bool("boot_rom_enabled").unwrap(),
          boot_rom_path: c.get_str("boot_rom_path").unwrap()
        },
      Err(why) => panic!("Failed to load config file {}: {}", path, why)
    }
  }
}

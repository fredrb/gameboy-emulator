use std::io::Read;
use std::fs::File;

#[derive(PartialEq)]
#[derive(Debug)]
enum CGB_Flag {
    RetroCompatible,
    NonRetroCompatible,
    Unkown,
}

impl CGB_Flag {
    fn map(flag: u8) -> CGB_Flag {
        return match flag {
            0x80 => CGB_Flag::RetroCompatible,
            0xC0 => CGB_Flag::NonRetroCompatible,
            _ => CGB_Flag::Unkown
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum SGB_Flag {
    Support,
    NoSupport,
    Unkown
}

impl SGB_Flag {
    fn map(flag: u8) -> SGB_Flag {
        return match flag {
            0x00 => SGB_Flag::NoSupport,
            0x03 => SGB_Flag::Support,
            _ => SGB_Flag::Unkown
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum CartridgeType {
    ROM_ONLY,
    MBC1,
    MBC1_RAM,
    MBC1_RAM_BATTERY,
    MBC2,
    MBC2_BATTERY,
    ROM_RAM,
    ROM_RAM_BATTERY,
    MMM01,
    MMM01_RAM,
    MMM01_RAM_BATTERY,
    MBC3_TIMER_BATTERY,
    MBC3_TIMER_RAM_BATTERY,
    MBC3,
    MBC3_RAM,
    MBC3_RAM_BATTERY,
    MBC5,
    MBC5_RAM,
    MBC5_RAM_BATTERY,
    MBC5_RUMBLE,
    MBC5_RUMBLE_RAM,
    MBC5_RUMBLE_RAM_BATTERY,
    MBC6,
    MBC7_SENSOR_RUMBLE_RAM_BATTERY,
    POCKET_CAMERA,
    BANDAI_TAMA5,
    HUC3,
    HUC1_RAM_BATTERY,
    Unkown,
}

impl CartridgeType {
    fn map(cartridge_type: u8) -> CartridgeType {
        return match cartridge_type {
            0x00 => CartridgeType::ROM_ONLY,
            0x01 => CartridgeType::MBC1,
            0x02 => CartridgeType::MBC1_RAM,
            0x03 => CartridgeType::MBC1_RAM_BATTERY,
            0x05 => CartridgeType::MBC2,
            0x06 => CartridgeType::MBC2_BATTERY,
            0x08 => CartridgeType::ROM_RAM,
            0x09 => CartridgeType::ROM_RAM_BATTERY,
            0x0B => CartridgeType::MMM01,
            0x0C => CartridgeType::MMM01_RAM,
            0x0D => CartridgeType::MMM01_RAM_BATTERY,
            0x0F => CartridgeType::MBC3_TIMER_BATTERY,
            0x10 => CartridgeType::MBC3_TIMER_RAM_BATTERY,
            0x11 => CartridgeType::MBC3,
            0x12 => CartridgeType::MBC3_RAM,
            0x13 => CartridgeType::MBC3_RAM_BATTERY,
            0x19 => CartridgeType::MBC5,
            0x1A => CartridgeType::MBC5_RAM,
            0x1B => CartridgeType::MBC5_RAM_BATTERY,
            0x1C => CartridgeType::MBC5_RUMBLE,
            0x1D => CartridgeType::MBC5_RUMBLE_RAM,
            0x1E => CartridgeType::MBC5_RUMBLE_RAM_BATTERY,
            0x20 => CartridgeType::MBC6,
            0x22 => CartridgeType::MBC7_SENSOR_RUMBLE_RAM_BATTERY,
            0xFC => CartridgeType::POCKET_CAMERA,
            0xFD => CartridgeType::BANDAI_TAMA5,
            0xFE => CartridgeType::HUC3,
            0xFF => CartridgeType::HUC1_RAM_BATTERY,
            _ => CartridgeType::Unkown
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum RomSize {
    _32_KByte_2_banks,
    _64_KByte_4_banks,
    _128_KByte_8_banks,
    _256_KByte_16_banks,
    _512_KByte_32_banks,
    _1_MByte_64_banks,
    _2_MByte_128_banks,
    _4_MByte_256_banks,
    _8_MByte_512_banks,
    _1_1_MByte_72_banks,
    _1_2_MByte_80_banks,
    _1_5_MByte_96_banks,
    Unkown
}

impl RomSize {
    fn map(byte: u8) -> RomSize {
        return match byte {
            0x00 => RomSize::_32_KByte_2_banks,
            0x01 => RomSize::_64_KByte_4_banks,
            0x02 => RomSize::_128_KByte_8_banks,
            0x03 => RomSize::_256_KByte_16_banks,
            0x04 => RomSize::_512_KByte_32_banks,
            0x05 => RomSize::_1_MByte_64_banks,
            0x06 => RomSize::_2_MByte_128_banks,
            0x07 => RomSize::_4_MByte_256_banks,
            0x08 => RomSize::_8_MByte_512_banks,
            0x52 => RomSize::_1_1_MByte_72_banks,
            0x53 => RomSize::_1_2_MByte_80_banks,
            0x54 => RomSize::_1_5_MByte_96_banks,
            _ => RomSize::Unkown,
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum RamSize {
    NONE,
    _2_KBytes,
    _8_KBytes,
    _32_KBytes,
    _128_KBytes,
    _64_KBytes,
    Unkown
}

impl RamSize {
    fn map(byte: u8) -> RamSize {
        return match byte {
            0x00 => RamSize::NONE,
            0x01 => RamSize::_2_KBytes,
            0x02 => RamSize::_8_KBytes,
            0x03 => RamSize::_32_KBytes,
            0x04 => RamSize::_128_KBytes,
            0x05 => RamSize::_64_KBytes,
            _ => RamSize::Unkown
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum DestinationCode {
    Japanese,
    NonJapanese,
    Unkown
}

impl DestinationCode {
    fn map(byte: u8) -> DestinationCode {
        return match byte {
            0x00 => DestinationCode::Japanese,
            0x01 => DestinationCode::NonJapanese,
            _ => DestinationCode::Unkown
        }
    }
}

pub struct Cartridge {
    // title: String,
    manufacturer_code: [u8;4],
    cgb_flag: CGB_Flag,
    license_code: [u8;2],
    sgb_flag: SGB_Flag,
    cartridge_type: CartridgeType,
    rom_size: RomSize,
    ram_size: RamSize,
    destination_code: DestinationCode,
    rom_version_num: u8,
    header_checksum: u8,
    global_checksum: u16,
    pub content: Vec<u8>
}

pub fn _8bit_to_16bit(left: u8, right: u8) -> u16 {
    let res = (left as u16) << 8;
    return res + (right as u16);
}

impl Cartridge {
    pub fn from_file(path: &str) -> Self {
        let mut file = match File::open(path) {
            Err(why) => panic!("Couldn't open {}: {}", path, why),
            Ok(f) => f
        };
    
        let mut buf: Vec<u8> = Vec::new();
        return match file.read_to_end(&mut buf) {
            Err(why) => panic!("Couldn't read byte stream: {}", why),
            Ok(size) => {
                println!("Read {} bytes of data", size);
                Cartridge::new(&buf)
            }
        };
    }

    pub fn new(cartridge_raw: &Vec<u8>) -> Self {
        return Cartridge {
            manufacturer_code: [0;4],
            cgb_flag: CGB_Flag::map(cartridge_raw[0x143]),
            license_code: [0;2],
            sgb_flag: SGB_Flag::map(cartridge_raw[0x146]),
            cartridge_type: CartridgeType::map(cartridge_raw[0x147]),
            rom_size: RomSize::map(cartridge_raw[0x148]),
            ram_size: RamSize::map(cartridge_raw[0x149]),
            destination_code: DestinationCode::map(cartridge_raw[0x14A]),
            rom_version_num: cartridge_raw[0x14C],
            header_checksum: cartridge_raw[0x14D],
            global_checksum: _8bit_to_16bit(cartridge_raw[0x14E], cartridge_raw[0x14F]),
            content: cartridge_raw.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_intr_01_headers() {
        let test_path = String::from("./res/test/01-special.gb");
        let loaded = Cartridge::from_file(&test_path);

        assert_eq!(loaded.cgb_flag, CGB_Flag::RetroCompatible);
        assert_eq!(loaded.cartridge_type, CartridgeType::MBC1);
        assert_eq!(loaded.rom_size, RomSize::_32_KByte_2_banks);
        assert_eq!(loaded.ram_size, RamSize::NONE);
        assert_eq!(loaded.header_checksum, 0x66);
        assert_eq!(loaded.global_checksum, 0x4deb);

    }
}
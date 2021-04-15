use crate::cpu::cpu::CPU;
use crate::debug::logger::{LogEvents, LoggableComponent, LoggerClient};
use crate::debug::{debugger::Debuggable, gb_debugger::DebuggerState};
use crate::debug::{input::DebuggerInput, logger::LogMessage, ui::Terminal};
use crate::external::boot_rom_loader;
use crate::external::cartridge::Cartridge;
use crate::gb_config::gb_config;
use crate::mmu::VirtualMemory;

pub struct Gameboy {
    pub cpu: CPU,
    pub mmu: VirtualMemory,
    pub cfg: gb_config,
    pub cartridge: Option<Cartridge>,
    pub terminal: Terminal,
    pub input: DebuggerInput,
    pub state: DebuggerState,

    logger_client: LoggerClient,
}

pub trait ExternalHook {
    fn when_started(gb: &mut Gameboy);
    fn before_tick(gb: &mut Gameboy);
    fn after_tick(gb: &mut Gameboy);
}

impl Gameboy {
    pub fn new(cfg: gb_config, logger_client: LoggerClient) -> Self {
        Gameboy {
            cpu: CPU::new(false),
            mmu: VirtualMemory::new(),
            cartridge: None,
            terminal: Terminal::new(8),
            input: DebuggerInput::new(),
            state: DebuggerState::new(),
            cfg,
            logger_client,
        }
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.logger_client
            .send((LogEvents::Initializing, String::from("Initializing ROM")));
        match self.mmu.load_rom(cartridge.content.as_slice()) {
            Ok(size) => self.logger_client.send((
                LogEvents::Initializing,
                format!("{} bytes loaded into memory", size),
            )),
            Err(why) => panic!("Failed to load ROM {}", why), // @TODO HANDLE PANICS
        }

        self.cartridge = Some(cartridge);
        if self.cfg.boot_rom_enabled {
            let boot_rom = boot_rom_loader::load_boot_rom(&self.cfg.boot_rom_path);
            match self.mmu.load_rom(boot_rom.as_slice()) {
                Ok(size) => self.logger_client.send((
                    LogEvents::Initializing,
                    format!("{} bytes loaded into boot rom area", size),
                )),
                Err(why) => panic!("Failed to load boot rom {}", why),
            }
        } else {
            self.cpu.reg.pc = 0x100;
        }
    }

    pub fn start(&mut self) {
        self.logger_client.send((
            LogEvents::Initializing,
            String::from("Starting GB main loop"),
        ));
        if self.cfg.debug_mode {
            self.state.breakpoints.push(self.cfg.initial_breakpoint);
            self.on_started();
        }

        loop {
            if self.cfg.debug_mode {
                if self.should_stop(self.cpu.reg.pc as u16) {
                    self.on_breakpoint(self.cpu.reg.pc as u16);
                    if self.state.log_next {
                        self.logger_client
                            .send((LogEvents::DebugLoggerOn, String::from("Debug logger on")));
                    } else {
                        self.logger_client
                            .send((LogEvents::DebugLoggerOff, String::from("Debug logger off")));
                    }
                }
            }

            let mut messages: Vec<LogMessage> = vec![];
            self.cpu.tick(&mut self.mmu);
            messages.extend(self.cpu.dump_log_messages());
            messages.extend(self.mmu.dump_log_messages());

            for m in messages {
                self.logger_client.send(m);
            }

            // for s in self.cpu.reg.print_registers() {
            //   self.logger_client.send((LogEvents::Register, format!("\t[REG] {}", s)));
            // }
        }
    }
}

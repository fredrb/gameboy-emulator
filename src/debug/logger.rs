use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::gb_config::gb_config;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub type LogMessage = (LogEvents, String);

pub trait LoggableComponent {
  fn dump_log_messages(&mut self) -> Vec<LogMessage>;
}

#[derive(Copy)]
pub enum LogEvents {
  Initializing,
  Tick,
  MemorySave,
  MemoryFetch,
  VramSave,
  Decoding,
  Snapshot,
  Exit,
  DebugLoggerOn,
  DebugLoggerOff,
  Register,
}

impl Clone for LogEvents {
  fn clone(&self) -> Self {
    return *self;
  }
}

pub struct Logger {
  cfg: gb_config,
  chout: Receiver<LogMessage>,
  chin: Sender<LogMessage>,

  open_file: File,
  debug_logger: bool,
}

impl Logger {
  pub fn new(cfg: gb_config) -> Self {
    let (chin, chout) = mpsc::channel();
    // @TODO: Use config for log file name
    let open_file = File::create(&Path::new("./log")).unwrap();
    Logger { 
      cfg,
      chin,
      chout,
      open_file,
      debug_logger: false,
    }
  }

  pub fn make_client(&self) -> LoggerClient {
    let client_chin = self.chin.clone();
    return LoggerClient::new(client_chin);
  }

  pub fn poll_message(&mut self) -> Result<(),String> {
    match self.chout.recv() {
      Ok(m) => Ok(self.process_message(m)),
      Err(e) => Err(format!("failed to poll message {}", e))
    }
  }

  fn log_message(&mut self, msg: String) {
    // @TODO: Handle result
    self.open_file.write_all(String::from(format!("{}\n", msg)).as_bytes());
  }

  fn log_terminal(&mut self, msg: String) {
    println!("{}", msg);
  }

  fn print_debug(&mut self, msg: String) {
    if self.debug_logger {
      println!("{}", msg);
    }
  }

  fn process_message(&mut self, msg: LogMessage) {
    match msg.0 {
      LogEvents::VramSave => self.print_debug(msg.1),
      LogEvents::Tick => self.print_debug(msg.1),
      LogEvents::MemoryFetch => self.print_debug(msg.1),
      LogEvents::MemorySave => self.print_debug(msg.1),
      LogEvents::Register => self.print_debug(msg.1),
      LogEvents::DebugLoggerOn => self.debug_logger = true,
      LogEvents::DebugLoggerOff => self.debug_logger = false,
      _ => ()
    };
  }
}

pub struct LoggerClient {
  chin: Sender<LogMessage>
}

impl LoggerClient {
  pub fn new(chin: Sender<LogMessage>) -> Self {
    LoggerClient { chin }
  }

  pub fn send(&self, message: LogMessage) {
    match self.chin.send(message) {
      Ok(_) => (),
      Err(why) => println!("Failed to send message to channel {}", why)
    }
  }
}
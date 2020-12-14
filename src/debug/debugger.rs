pub trait Debuggable {
  fn on_started(&mut self);

  fn on_breakpoint(&mut self, addr: u16);

  fn should_stop(&self, addr: u16) -> bool;
}
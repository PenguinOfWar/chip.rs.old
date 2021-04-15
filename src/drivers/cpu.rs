// I think i get this now and its cool

#[derive(Debug)]
pub struct Cpu {
  pub working: bool,
  pub running: bool,
}

pub trait CpuContext {
  fn new() -> Self;
  fn set_running(self, running: bool) -> Self;
  fn tick(self) -> Result<Cpu, String>;
}

impl CpuContext for Cpu {
  fn new() -> Cpu {
    Cpu {
      running: false,
      working: false,
    }
  }
  fn set_running(mut self, running: bool) -> Cpu {
    self.running = running;
    return self;
  }
  fn tick(mut self) -> Result<Cpu, String> {
    self.working = true;

    // do stuff
    log::info!("tick");

    self.working = false;

    return Ok(self);
  }
}

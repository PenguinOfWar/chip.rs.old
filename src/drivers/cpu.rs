#[derive(Debug)]
pub struct Cpu {
  pub working: bool,
  pub running: bool,
}

pub trait CpuContext {
  fn new() -> Self;
  fn set_running(self, running: bool) -> Self;
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
}

pub fn init() -> Cpu {
  CpuContext::new()
}

pub fn tick() -> Result<(), String> {
  Ok(render())
}

fn render() {
  log::info!("tick");
}

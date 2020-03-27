use std::time::Duration;

#[derive(Debug, Default)]
pub struct GameTime {
  pub elapsed: Duration,
  pub dt: Duration,
}

impl GameTime {
  pub fn capture_time(&mut self, dt: Duration) {
    self.elapsed += dt;
    self.dt = dt;
  }
}

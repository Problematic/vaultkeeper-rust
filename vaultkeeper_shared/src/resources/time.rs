use std::time::Duration;

#[derive(Debug, Default)]
pub struct Time {
  /// Time in seconds to complete last frame
  pub dt: Duration,

  /// Time the frame started, in seconds since level was loaded
  pub elapsed: Duration,
}

impl Time {
  pub fn capture(&mut self, dt: Duration) {
    self.elapsed += dt;
    self.dt = dt;
  }
}

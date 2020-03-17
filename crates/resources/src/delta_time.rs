use std::time::Duration;

#[derive(Debug, Default, Clone, Copy)]
pub struct DeltaTime(pub Duration);

impl DeltaTime {
  pub fn as_secs_f32(&self) -> f32 {
    self.0.as_secs_f32()
  }

  pub fn as_duration(&self) -> Duration {
    self.0
  }
}

impl std::ops::Deref for DeltaTime {
  type Target = Duration;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

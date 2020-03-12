use std::time::Duration;

#[derive(Debug, Default, Clone, Copy)]
pub struct DeltaTime(pub Duration);

impl std::ops::Deref for DeltaTime {
  type Target = Duration;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

use std::time::Duration;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Lifetime {
  Frames(u32),
  GameTime(Duration),
  // Turns(u32),
  // Attempts(u32),
}

use crate::components::{Position, Target};
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Navigation {
  pub goal: Option<Target>,
  pub path: VecDeque<Position>,
}

impl Navigation {
  pub fn next(&mut self) -> Option<Position> {
    self.path.pop_front()
  }

  pub fn is_empty(&self) -> bool {
    self.path.is_empty()
  }
}

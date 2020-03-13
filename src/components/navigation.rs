use super::{Position, Zone};
use specs::{prelude::*, Component};
use std::collections::VecDeque;

#[derive(Component, Debug)]
pub struct Navigation {
  pub goal: Option<Box<dyn Zone>>,
  pub path: VecDeque<Position>,
}

impl Default for Navigation {
  fn default() -> Self {
    Self {
      goal: None,
      path: VecDeque::default(),
    }
  }
}

impl Navigation {
  #[must_use]
  pub fn next(&mut self) -> Option<Position> {
    self.path.pop_front()
  }

  pub fn at_goal(&self, position: Position) -> bool {
    self.goal.is_none() || self.goal.as_ref().unwrap().contains(position)
  }
}

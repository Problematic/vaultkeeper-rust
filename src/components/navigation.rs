use crate::components::Position;
use specs::{prelude::*, Component};
use std::collections::HashSet;

#[derive(Component, Debug, Default)]
pub struct Navigation {
  pub goal: HashSet<Position>,
  pub path: Vec<Position>,
}

impl Navigation {
  #[must_use]
  pub fn needs_path(&self) -> bool {
    !self.goal.is_empty()
      && (self.path.is_empty() || !self.goal.contains(self.path.last().unwrap()))
  }
}

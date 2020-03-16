#![allow(clippy::module_name_repetitions)]

use super::utils;
use super::Position;

pub trait Zone: Send + Sync + std::fmt::Debug {
  fn contains(&self, position: Position) -> bool;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct RadialZone(pub Position, pub i32);

impl Zone for RadialZone {
  fn contains(&self, position: Position) -> bool {
    utils::chebyshev_dist(self.0, position) <= self.1
  }
}

impl Zone for Position {
  fn contains(&self, position: Position) -> bool {
    position == *self
  }
}

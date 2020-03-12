#![allow(clippy::module_name_repetitions)]

use crate::components::Position;
use crate::utils;

pub trait Zone: Send + Sync + std::fmt::Debug {
  fn contains(&self, position: Position) -> bool;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct RadialZone(pub Position, pub i32);

impl Zone for RadialZone {
  fn contains(&self, position: Position) -> bool {
    utils::geom::chebyshev_dist(self.0, position) <= self.1
  }
}

impl Zone for Position {
  fn contains(&self, position: Position) -> bool {
    position == *self
  }
}

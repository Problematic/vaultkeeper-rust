use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Input {
  MoveNorth,
  MoveEast,
  MoveSouth,
  MoveWest,
}
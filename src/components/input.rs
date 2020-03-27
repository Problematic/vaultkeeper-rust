use super::Direction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Input {
  Move(Direction),
  Cancel,
  TogglePause,
  Wait,
}

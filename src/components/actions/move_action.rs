use crate::components::Direction;

#[derive(Debug)]
pub struct MoveAction {
  pub direction: Direction,
}

impl MoveAction {
  pub fn new(direction: Direction) -> Self {
    Self { direction }
  }
}

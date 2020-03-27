use crate::components::Direction;

pub struct AttackAction {
  pub direction: Direction,
}

impl AttackAction {
  pub fn new(direction: Direction) -> Self {
    Self { direction }
  }
}

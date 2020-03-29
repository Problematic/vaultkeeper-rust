use crate::components::Target;

#[derive(Debug)]
pub struct MoveAction {
  pub target: Target,
}

impl MoveAction {
  pub fn new(target: Target) -> Self {
    Self { target }
  }
}

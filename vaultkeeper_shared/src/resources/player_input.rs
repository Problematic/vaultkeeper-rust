#[derive(Debug, PartialEq, Eq)]
pub enum MoveDirection {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug, Default)]
pub struct PlayerInput {
  pub move_dir: Option<MoveDirection>,
}

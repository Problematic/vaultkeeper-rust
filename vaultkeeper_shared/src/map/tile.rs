pub trait Tile: Default + Clone {
  fn set_blocked(&mut self, blocked: bool);
  fn is_blocked(&self) -> bool;
}

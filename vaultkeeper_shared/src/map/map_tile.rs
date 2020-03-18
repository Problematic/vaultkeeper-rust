use super::Tile;

#[derive(Debug, Default, Clone)]
pub struct MapTile {
  is_blocked: bool,
}

impl Tile for MapTile {
  fn set_blocked(&mut self, blocked: bool) {
    self.is_blocked = blocked;
  }

  fn is_blocked(&self) -> bool {
    self.is_blocked
  }
}

use crate::components::Appearance;
use bracket_lib::prelude::*;
use legion::prelude::Entity;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TileType {
  Open,
  Solid,
}

impl TileType {
  pub fn appearance(self) -> Appearance {
    use TileType::*;

    match self {
      Open => Appearance::new('·', (0x2a, 0x24, 0x2b), BLACK),
      Solid => Appearance::new('#', (0x84, 0x7e, 0x87), BLACK),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Tile {
  pub kind: TileType,
  /// a tile can only ever have a maximum of one "occupant", which is
  /// defined as an actor-type entity (player, monster, etc)
  pub occupant: Option<Entity>,
  pub is_revealed: bool,
}

impl Tile {
  // TODO: cache this?
  pub fn appearance(&self) -> Appearance {
    self.kind.appearance()
  }

  pub fn is_walkable(&self) -> bool {
    self.kind == TileType::Open && self.occupant.is_none()
  }

  pub fn is_opaque(&self) -> bool {
    self.kind == TileType::Solid
  }
}

impl Default for Tile {
  fn default() -> Self {
    Self {
      kind: TileType::Open,
      occupant: None,
      is_revealed: false,
    }
  }
}

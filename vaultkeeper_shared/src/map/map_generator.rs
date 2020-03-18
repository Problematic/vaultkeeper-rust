use super::{Tile, WorldMap};

pub trait MapGenerator<T: Tile> {
  fn new() -> Self;

  fn build(&mut self) -> WorldMap<T>;
}

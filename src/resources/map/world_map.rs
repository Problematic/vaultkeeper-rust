use super::Tile;
use crate::components::{Appearance, Position};
use bracket_lib::prelude::*;
use std::ops;

pub struct WorldMap {
  width: i32,
  height: i32,
  tiles: Vec<Tile>,
  pub rooms: Vec<Rect>,
}

impl WorldMap {
  pub fn new(width: i32, height: i32) -> Self {
    assert!(width > 0);
    assert!(height > 0);

    Self {
      width,
      height,
      tiles: vec![Tile::default(); (width * height) as usize],
      rooms: Vec::with_capacity(20),
    }
  }

  pub fn new_from(width: i32, height: i32, source: Tile) -> Self {
    assert!(width > 0);
    assert!(height > 0);

    Self {
      width,
      height,
      tiles: vec![source; (width * height) as usize],
      rooms: Vec::with_capacity(20),
    }
  }

  pub fn width(&self) -> i32 {
    self.width
  }

  pub fn height(&self) -> i32 {
    self.height
  }

  pub fn xy_to_idx(&self, (x, y): (i32, i32)) -> usize {
    ((y * self.width) + x) as usize
  }

  pub fn idx_to_xy(&self, idx: usize) -> (i32, i32) {
    let idx = idx as i32;
    (idx % self.width, idx / self.width)
  }

  pub fn render(&self, term: &mut BTerm) {
    for idx in 0..self.tiles.len() {
      let (x, y) = self.idx_to_xy(idx);
      let tile = &self.tiles[idx];

      let Appearance { glyph, fg, bg } = tile.appearance();

      term.set(x, y, fg, bg, glyph);
    }
  }
}

impl ops::Index<usize> for WorldMap {
  type Output = Tile;

  fn index(&self, idx: usize) -> &Self::Output {
    &self.tiles[idx]
  }
}

impl ops::Index<(i32, i32)> for WorldMap {
  type Output = Tile;

  fn index(&self, pos: (i32, i32)) -> &Self::Output {
    &self.tiles[self.xy_to_idx(pos)]
  }
}

impl ops::Index<Position> for WorldMap {
  type Output = Tile;

  fn index(&self, pos: Position) -> &Self::Output {
    &self.tiles[self.xy_to_idx((pos.x, pos.y))]
  }
}

impl ops::IndexMut<usize> for WorldMap {
  fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
    &mut self.tiles[idx]
  }
}

impl ops::IndexMut<(i32, i32)> for WorldMap {
  fn index_mut(&mut self, pos: (i32, i32)) -> &mut Self::Output {
    let idx = self.xy_to_idx(pos);
    &mut self.tiles[idx]
  }
}

impl ops::IndexMut<Position> for WorldMap {
  fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
    let idx = self.xy_to_idx((pos.x, pos.y));
    &mut self.tiles[idx]
  }
}

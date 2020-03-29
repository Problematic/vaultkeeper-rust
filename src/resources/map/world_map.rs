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

  /// Renders only revealed tiles (entities and visible tiles are in a
  /// separate rendering pass)
  pub fn render(&self, batch: &mut DrawBatch) {
    for idx in 0..self.tiles.len() {
      let tile = &self.tiles[idx];

      if !tile.is_revealed {
        continue;
      }

      let (x, y) = self.idx_to_xy(idx);
      let Appearance { glyph, colors } = tile.appearance();
      let ColorPair { fg, bg } = colors;

      batch.set(
        Position::new(x, y),
        ColorPair::new(fg.desaturate().lerp(bg, 0.65), bg),
        glyph,
      );
    }
  }

  pub fn get_neighbors(&self, pos: Position) -> Vec<Position> {
    let mut neighbors = Vec::with_capacity(8);

    let Position { x, y } = pos;

    for dx in -1..=1 {
      for dy in -1..=1 {
        let p = Position::new(x + dx, y + dy);
        if self[p].is_walkable() {
          neighbors.push(p);
        }
      }
    }

    neighbors
  }
}

impl Algorithm2D for WorldMap {
  fn dimensions(&self) -> Point {
    Point::new(self.width, self.height)
  }

  fn in_bounds(&self, pos: Point) -> bool {
    pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
  }
}

impl BaseMap for WorldMap {
  fn is_opaque(&self, idx: usize) -> bool {
    self.tiles[idx].is_opaque()
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

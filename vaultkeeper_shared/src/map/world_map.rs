use super::Tile;
use crate::Render;
use bracket_lib::prelude::*;
use components::Position;
use std::ops;

const NEIGHBORS: [(i32, i32); 8] = [
  (0, -1),
  (1, -1),
  (1, 0),
  (1, 1),
  (0, 1),
  (-1, 1),
  (-1, 0),
  (-1, -1),
];

pub struct WorldMap<T>
where
  T: Tile,
{
  width: i32,
  height: i32,
  tiles: Vec<T>,
}

impl<T> WorldMap<T>
where
  T: Tile,
{
  pub fn new(width: i32, height: i32) -> Self {
    assert!(width > 0);
    assert!(height > 0);

    Self {
      width,
      height,
      tiles: vec![T::default(); (width * height) as usize],
    }
  }

  pub fn new_from(width: i32, height: i32, source: T) -> Self
  where
    T: Clone,
  {
    assert!(width > 0);
    assert!(height > 0);

    Self {
      width,
      height,
      tiles: vec![source; (width * height) as usize],
    }
  }

  pub fn height(&self) -> i32 {
    self.height
  }

  pub fn width(&self) -> i32 {
    self.width
  }

  pub fn xy_to_idx(&self, (x, y): (i32, i32)) -> usize {
    ((y * self.width) + x) as usize
  }

  pub fn idx_to_xy(&self, idx: usize) -> (i32, i32) {
    let idx = idx as i32;
    (idx % self.width, idx / self.width)
  }

  pub fn contains(&self, (x, y): (i32, i32)) -> bool {
    x >= 0 && x < self.width && y >= 0 && y < self.height
  }

  pub fn neighbors(&self, (x, y): (i32, i32)) -> Vec<(i32, i32)> {
    NEIGHBORS
      .iter()
      .map(|(dx, dy)| (x + dx, y + dy))
      .filter(|pos| self.contains(*pos) && !self[*pos].is_blocked())
      .collect()
  }
}

impl<T> Render for WorldMap<T>
where
  T: Tile,
{
  fn render(&self, term: &mut BTerm) {
    for idx in 0..self.tiles.len() {
      let (x, y) = self.idx_to_xy(idx);
      let tile = &self.tiles[idx];

      term.set(
        x,
        y,
        DARKGREY,
        BLACK,
        if tile.is_blocked() { b'#' } else { b'.' },
      );
    }
  }
}

impl<T> ops::Index<usize> for WorldMap<T>
where
  T: Tile,
{
  type Output = T;

  fn index(&self, idx: usize) -> &Self::Output {
    &self.tiles[idx]
  }
}

impl<T> ops::Index<(i32, i32)> for WorldMap<T>
where
  T: Tile,
{
  type Output = T;

  fn index(&self, pos: (i32, i32)) -> &Self::Output {
    &self.tiles[self.xy_to_idx(pos)]
  }
}

impl<T> ops::Index<Position> for WorldMap<T>
where
  T: Tile,
{
  type Output = T;

  fn index(&self, pos: Position) -> &Self::Output {
    &self.tiles[self.xy_to_idx((pos.x, pos.y))]
  }
}

impl<T> ops::IndexMut<usize> for WorldMap<T>
where
  T: Tile,
{
  fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
    &mut self.tiles[idx]
  }
}

impl<T> ops::IndexMut<(i32, i32)> for WorldMap<T>
where
  T: Tile,
{
  fn index_mut(&mut self, pos: (i32, i32)) -> &mut Self::Output {
    let idx = self.xy_to_idx(pos);
    &mut self.tiles[idx]
  }
}

impl<T> ops::IndexMut<Position> for WorldMap<T>
where
  T: Tile,
{
  fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
    let idx = self.xy_to_idx((pos.x, pos.y));
    &mut self.tiles[idx]
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::map::MapTile;

  #[test]
  fn test_neighbors() {
    let mut m = WorldMap::<MapTile>::new(5, 5);

    assert_eq!(
      m.neighbors((2, 2)),
      vec![
        (2, 1),
        (3, 1),
        (3, 2),
        (3, 3),
        (2, 3),
        (1, 3),
        (1, 2),
        (1, 1),
      ]
    );

    m[(1, 1)].set_blocked(true);

    assert_eq!(
      m.neighbors((2, 2)),
      vec![(2, 1), (3, 1), (3, 2), (3, 3), (2, 3), (1, 3), (1, 2),]
    );
  }
}

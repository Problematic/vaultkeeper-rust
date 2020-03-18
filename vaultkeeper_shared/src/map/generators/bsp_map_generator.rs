use crate::map::{MapGenerator, Tile, WorldMap};
use bracket_lib::prelude::Rect;
use rand::{
  distributions::{Distribution, Standard},
  Rng,
};

enum Direction {
  Horizontal,
  Vertical,
}

impl Distribution<Direction> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
    if rng.gen_range(0.0, 1.0) < 0.5_f32 {
      Direction::Horizontal
    } else {
      Direction::Vertical
    }
  }
}

trait Partitionable: Sized {
  fn partition(&self, alpha: f32) -> (Self, Self);
}

impl Partitionable for Rect {
  fn partition(&self, alpha: f32) -> (Self, Self) {
    let direction: Direction = rand::random();

    match direction {
      Direction::Horizontal => {
        let split = self.y1 + (self.height() as f32 * alpha) as i32;

        (
          Rect::with_exact(self.x1, self.y1, self.x2, split),
          Rect::with_exact(self.x1, split + 1, self.x2, self.y2),
        )
      }
      Direction::Vertical => {
        let split = self.x1 + (self.width() as f32 * alpha) as i32;

        (
          Rect::with_exact(self.x1, self.y1, split, self.y2),
          Rect::with_exact(split + 1, self.y1, self.x2, self.y2),
        )
      }
    }
  }
}

type Depth = u8;

enum BSPTree<T>
where
  T: Partitionable,
{
  Leaf(Depth, T),
  Node(Depth, Box<BSPTree<T>>, Box<BSPTree<T>>),
  None,
}

impl<T> BSPTree<T>
where
  T: Partitionable,
{
  pub fn new(data: T) -> Self {
    Self::Leaf(0, data)
  }

  pub fn partition(&mut self, alpha: f32) {
    use BSPTree::{Leaf, Node};

    *self = match std::mem::replace(self, BSPTree::None) {
      Leaf(depth, data) => {
        let (left, right) = data.partition(alpha);

        Node(
          depth,
          Box::new(Leaf(depth + 1, left)),
          Box::new(Leaf(depth + 1, right)),
        )
      }
      Node(depth, mut left, mut right) => {
        left.partition(alpha);
        right.partition(alpha);

        Node(depth, left, right)
      }
      otherwise => otherwise,
    }
  }
}

#[derive(Default)]
pub struct BSPMapGenerator<T>
where
  T: Tile,
{
  width: i32,
  height: i32,
  alpha: f32,
  iterations: u8,
  impassible_borders: bool,
  _tile: std::marker::PhantomData<T>,
}

impl<T> BSPMapGenerator<T>
where
  T: Tile,
{
  pub fn with_dimensions(mut self, width: i32, height: i32) -> Self {
    self.width = width;
    self.height = height;

    self
  }

  pub fn with_iterations(mut self, iterations: u8) -> Self {
    self.iterations = iterations;

    self
  }

  pub fn with_impassible_borders(mut self, impassible: bool) -> Self {
    self.impassible_borders = impassible;

    self
  }

  pub fn with_alpha(mut self, alpha: f32) -> Self {
    self.alpha = alpha;

    self
  }
}

impl<T> MapGenerator<T> for BSPMapGenerator<T>
where
  T: Tile,
{
  fn new() -> Self {
    Self {
      width: 80,
      height: 60,
      alpha: 0.5,
      iterations: 4,
      impassible_borders: true,

      _tile: std::marker::PhantomData,
    }
  }

  fn build(&mut self) -> WorldMap<T> {
    let mut map: WorldMap<T> = WorldMap::new(self.width, self.height);

    let mut tree = BSPTree::new(Rect::with_size(0, 0, self.width, self.height));

    for _ in 0..self.iterations {
      tree.partition(self.alpha);
    }

    if self.impassible_borders {
      for y in 0..self.height {
        for x in 0..self.width {
          if x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1 {
            map[(x, y)].set_blocked(true);
          }
        }
      }
    }

    map
  }
}

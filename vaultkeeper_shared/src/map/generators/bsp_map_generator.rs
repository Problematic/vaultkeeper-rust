use crate::map::{MapGenerator, Tile, WorldMap};
use crate::utils::{BSPTree, Partition};
use bracket_lib::prelude::*;
use components::Position;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

pub struct Region {
  bounds: Rect,
  min_size: (i32, i32),
  jitter: f32,
}

impl Region {
  pub fn new(bounds: Rect, min_size: (i32, i32), jitter: f32) -> Self {
    Self {
      bounds,
      min_size,
      jitter,
    }
  }

  pub fn carve<T>(&self, rng: &mut ThreadRng, map: &mut WorldMap<T>, room_size: f32) -> Rect
  where
    T: Tile,
  {
    use std::cmp::{max, min};

    let width = max(
      (self.bounds.width() as f32 * room_size) as i32,
      min(self.min_size.0, self.bounds.width()),
    );
    let height = max(
      (self.bounds.height() as f32 * room_size) as i32,
      min(self.min_size.1, self.bounds.height()),
    );

    let gap_x = max(self.bounds.width() - width, 0);
    let gap_y = max(self.bounds.height() - height, 0);

    let x = if gap_x == 0 {
      self.bounds.x1
    } else {
      self.bounds.x1 + rng.gen_range(0, gap_x)
    };
    let y = if gap_y == 0 {
      self.bounds.y1
    } else {
      self.bounds.y1 + rng.gen_range(0, gap_y)
    };

    let rect = Rect::with_size(x, y, width, height);

    rect.for_each(|pos| {
      map[pos].set_blocked(false);
    });

    rect
  }
}

impl Partition for Region {
  fn partition(&self) -> Option<(Self, Self)> {
    let mut rng = rand::thread_rng();

    let should_split =
      self.bounds.width() >= self.min_size.0 && self.bounds.height() >= self.min_size.1;

    if !should_split {
      return None;
    }

    let split_horizontal = if self.bounds.height() < self.min_size.1 {
      false
    } else if self.bounds.width() < self.min_size.0 {
      true
    } else {
      rng.gen_ratio(1, 2)
    };

    let jitter = 0.5 + rng.gen_range(-self.jitter, self.jitter);

    if split_horizontal {
      let split = self.bounds.y1 + (self.bounds.height() as f32 * jitter) as i32;

      Some((
        Self::new(
          Rect::with_exact(self.bounds.x1, self.bounds.y1, self.bounds.x2, split),
          self.min_size,
          self.jitter,
        ),
        Self::new(
          Rect::with_exact(self.bounds.x1, split + 1, self.bounds.x2, self.bounds.y2),
          self.min_size,
          self.jitter,
        ),
      ))
    } else {
      let split = self.bounds.x1 + (self.bounds.width() as f32 * jitter) as i32;

      Some((
        Self::new(
          Rect::with_exact(self.bounds.x1, self.bounds.y1, split, self.bounds.y2),
          self.min_size,
          self.jitter,
        ),
        Self::new(
          Rect::with_exact(split + 1, self.bounds.y1, self.bounds.x2, self.bounds.y2),
          self.min_size,
          self.jitter,
        ),
      ))
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
  partition_jitter: f32,
  room_size: f32,
  filled: bool,
  iterations: u8,
  min_room_size: (i32, i32),
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

  pub fn with_partition_jitter(mut self, partition_jitter: f32) -> Self {
    self.partition_jitter = partition_jitter;

    self
  }

  pub fn with_room_size(mut self, room_size: f32) -> Self {
    self.room_size = room_size;

    self
  }

  pub fn with_min_room_size(mut self, min_room_size: (i32, i32)) -> Self {
    self.min_room_size = min_room_size;

    self
  }

  pub fn with_filled(mut self, filled: bool) -> Self {
    self.filled = filled;

    self
  }

  fn carve_corridor(&self, map: &mut WorldMap<T>, from: &Rect, to: &Rect) {
    let start = from.center();
    let end = to.center();

    let horizontal = Position::new(start.x + (end.x - start.x) / 2, start.y);

    let vert = Position::new(horizontal.x, end.y);

    {
      let line = Bresenham::new(start, horizontal);
      for point in line {
        map[point].set_blocked(false);
      }
    }

    {
      let line = Bresenham::new(horizontal, vert);
      for point in line {
        map[point].set_blocked(false);
      }
    }

    {
      let line = Bresenham::new(vert, end);
      for point in line {
        map[point].set_blocked(false);
      }
    }
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
      partition_jitter: 0.1,
      room_size: 0.8,
      min_room_size: (3, 3),
      filled: false,
      iterations: 4,
      impassible_borders: true,

      _tile: std::marker::PhantomData,
    }
  }

  fn build(&mut self) -> WorldMap<T> {
    let mut rng = rand::thread_rng();

    let mut map: WorldMap<T> = if self.filled {
      let mut t = T::default();
      t.set_blocked(true);

      WorldMap::new_from(self.width, self.height, t)
    } else {
      WorldMap::new(self.width, self.height)
    };

    let min_size = (self.min_room_size.0 * 2, self.min_room_size.1 * 2);

    let mut tree = BSPTree::new(Region::new(
      Rect::with_size(0, 0, self.width - 1, self.height - 1),
      min_size,
      self.partition_jitter,
    ));

    for _ in 0..self.iterations {
      tree.partition();
    }

    let mut rooms = Vec::new();

    tree.for_each(&mut |_depth, region: &Region| {
      let room = region.carve(&mut rng, &mut map, self.room_size);
      rooms.push(room);
    });

    for room in &rooms {
      let corridor_count = rng.gen_range(1, 3);
      for dest in rooms.choose_multiple(&mut rng, corridor_count) {
        self.carve_corridor(&mut map, room, dest);
      }
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

    map.rooms.extend(rooms);

    map
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tree_partitioning() {
    let rect = Rect::with_size(0, 0, 80, 60);
    let region = Region::new(rect, (1, 1), 0.0);
    let mut tree = BSPTree::new(region);

    tree.partition();
    tree.partition();
    tree.partition();

    let mut count = 0;

    tree.for_each(&mut |_depth, r| count += &r.bounds.point_set().len());

    assert_eq!(count, rect.point_set().len());
  }
}

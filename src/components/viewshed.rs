use crate::components::Position;
use std::collections::HashSet;

pub struct Viewshed {
  pub visible_tiles: HashSet<Position>,
  pub range: i32,
}

impl Viewshed {
  pub fn new(range: i32) -> Self {
    let cap = ((range + range) * (range + range)) as usize;

    Self {
      visible_tiles: HashSet::with_capacity(cap),
      range,
    }
  }
}

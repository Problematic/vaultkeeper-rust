use crate::components::Position;
use crate::resources::WorldMap;
use bracket_lib::prelude::{field_of_view_set, Algorithm2D};
use std::collections::HashSet;

#[derive(Debug)]
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

  pub fn update(&mut self, map: &mut WorldMap, position: Position) {
    self.visible_tiles.clear();

    let visible_tiles = field_of_view_set(position, self.range, map);

    self.visible_tiles.extend(visible_tiles);
    self.visible_tiles.retain(|p| map.in_bounds(*p));
  }
}

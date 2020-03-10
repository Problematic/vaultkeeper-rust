use crate::components::Position;
use specs::{prelude::*, Component};

#[derive(Component, Debug, Default)]
pub struct Viewshed {
  pub visible_entities: Vec<Entity>,
  pub visible_tiles: Vec<Position>,
}

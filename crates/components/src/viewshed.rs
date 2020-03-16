use super::Position;
use legion::prelude::Entity;

#[derive(Debug, Default)]
pub struct Viewshed {
  pub visible_entities: Vec<Entity>,
  pub visible_tiles: Vec<Position>,
}

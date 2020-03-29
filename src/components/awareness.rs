use legion::prelude::Entity;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Awareness {
  pub visible_entities: HashSet<Entity>,
}

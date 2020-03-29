use crate::components::Position;
use legion::prelude::Entity;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Memory {
  pub encounters: HashMap<Entity, Position>,
}

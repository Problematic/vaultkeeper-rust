use crate::components::{Appearance, Position};
use legion::prelude::Entity;

#[derive(Debug)]
pub struct Actor {
  pub id: Entity,
  pub pos: Position,
  pub appearance: Appearance,
}

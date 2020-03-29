use crate::components::Position;
use legion::prelude::Entity;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Target {
  Entity(Entity),
  Position(Position),
}

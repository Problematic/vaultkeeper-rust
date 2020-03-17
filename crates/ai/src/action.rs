use components::Position;
use legion::prelude::Entity;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Target {
  Entity(Entity),
  Position(Position),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Action {
  Goto(Target),
  Interact(Entity),
}

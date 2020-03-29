use legion::prelude::Entity;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intent {
  Attack(Entity),
  Flee,
}

use crate::components::{tags, Appearance, Lifetime, Position};
use legion::prelude::*;

pub trait Effect {
  fn spawn(&self, cmd: &mut CommandBuffer, position: Position) -> Entity;
}

#[derive(Debug)]
pub struct TimedEffect {
  pub appearance: Appearance,
  pub lifetime: Lifetime,
}

impl TimedEffect {
  pub fn new(appearance: Appearance, lifetime: Lifetime) -> Self {
    Self {
      appearance,
      lifetime,
    }
  }
}

impl Effect for TimedEffect {
  fn spawn(&self, cmd: &mut CommandBuffer, position: Position) -> Entity {
    cmd
      .start_entity()
      .with_tag((tags::Effect,))
      .with_component(position)
      .with_component(self.appearance)
      .with_component(self.lifetime)
      .build()
  }
}

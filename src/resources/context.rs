use crate::components::{Blackboard, Name, Navigation, Needs, Perception, Position};
use specs::prelude::*;
use std::time::Duration;
use utils;

#[derive(Debug)]
pub struct AICharacterData<'a> {
  pub entity: Entity,
  pub name: &'a Name,
  pub position: &'a Position,
  pub perception: &'a Perception,
  pub needs: &'a Needs,
}

#[allow(clippy::module_name_repetitions)]
pub struct AIContext<'a> {
  pub dt: Duration,
  pub agent: AICharacterData<'a>,
  pub entities: &'a Entities<'a>,
  pub world: &'a mut World,
  pub lazy_update: &'a LazyUpdate,
  pub blackboard: &'a mut Blackboard,
  pub navigation: &'a mut Navigation,
}

impl<'a> AIContext<'a> {
  pub fn distance_to_pos(&self, position: Position) -> i32 {
    utils::geom::chebyshev_dist(*self.agent.position, position)
  }
}

#[derive(Debug)]
pub struct CurrentAgent(pub Entity);

impl std::ops::Deref for CurrentAgent {
  type Target = Entity;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

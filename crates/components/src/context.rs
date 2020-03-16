use super::utils;
use super::{Blackboard, Name, Navigation, Needs, Perception, PointOfInterest, Position};
use specs::prelude::*;
use std::time::Duration;

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
  pub positions: &'a ReadStorage<'a, Position>,
  pub points_of_interest: &'a ReadStorage<'a, PointOfInterest>,
  pub blackboard: &'a mut Blackboard,
  pub navigation: &'a mut Navigation,
}

impl<'a> AIContext<'a> {
  pub fn distance_to_pos(&self, position: Position) -> i32 {
    utils::chebyshev_dist(*self.agent.position, position)
  }

  pub fn get_entity_pos(&self, entity: Option<Entity>) -> Option<Position> {
    if let Some(entity) = entity {
      self.positions.get(entity).copied()
    } else {
      None
    }
  }

  #[allow(dead_code)]
  pub fn distance_to_entity(&self, entity: Entity) -> Option<i32> {
    self
      .get_entity_pos(Some(entity))
      .map(|pos| self.distance_to_pos(pos))
  }
}

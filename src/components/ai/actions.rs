use super::considerations::*;
use super::{Blackboard, Decision, Need, Needs, PointOfInterest};
use crate::components::{Name, Perception, Position};
use crate::utils;
use ordered_float::NotNan;
use specs::{prelude::*, Component};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum AIInterest {
  Entity(Entity),
  Position(Position),
  BlackboardTarget,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum AIAction {
  Goto(AIInterest),
  Interact(Entity),
}

#[derive(Debug)]
pub struct AICharacterData<'a> {
  pub entity: Entity,
  pub name: &'a Name,
  pub position: &'a Position,
  pub perception: &'a Perception,
  pub needs: &'a Needs,
}

pub struct AIContext<'a> {
  pub agent: AICharacterData<'a>,
  pub entities: &'a Entities<'a>,
  pub positions: &'a ReadStorage<'a, Position>,
  pub points_of_interest: &'a ReadStorage<'a, PointOfInterest>,
  pub blackboard: &'a mut Blackboard,
}

impl<'a> AIContext<'a> {
  pub fn distance_to_pos(&self, position: Position) -> i32 {
    utils::geom::chebyshev_dist(*self.agent.position, position)
  }
}

#[derive(Component, Debug)]
pub struct CurrentAction(pub AIAction);

#[allow(clippy::module_name_repetitions)]
#[derive(Component, Debug)]
pub struct AvailableActions {
  pub decisions: Vec<Decision>,
}

impl Default for AvailableActions {
  fn default() -> Self {
    Self {
      decisions: vec![
        Decision {
          name: String::from("Go to social POI"),
          weight: 1.0,
          action: AIAction::Goto(AIInterest::BlackboardTarget),
          considerations: vec![
            Box::new(NearestPOIPicker { need: Need::Social }),
            Box::new(NeedConsideration { need: Need::Social }),
            Box::new(NearTargetConsideration { max_distance: 10.0 }),
          ],
        },
        Decision {
          name: String::from("Go to food POI"),
          weight: 1.0,
          action: AIAction::Goto(AIInterest::BlackboardTarget),
          considerations: vec![
            Box::new(NearestPOIPicker { need: Need::Hunger }),
            Box::new(NeedConsideration { need: Need::Hunger }),
            Box::new(NearTargetConsideration { max_distance: 10.0 }),
          ],
        },
      ],
    }
  }
}

impl AvailableActions {
  #[must_use]
  pub fn evaluate(&self, context: &mut AIContext) -> Option<(AIAction, String, f32)> {
    // TODO: return a lightweight handle to a globally-registered decision
    // so we don't have the clone the name and/or the decision itself
    self
      .decisions
      .iter()
      .map(|d| {
        log::debug!(
          "\u{250c} {} is evaluating '{}'...",
          context.agent.name,
          d.name
        );

        let score = d.score(context);

        log::debug!("\u{2514} Score => {}", score);

        (d.action, d.name.clone(), score)
      })
      .max_by_key(|r| NotNan::from(r.2))
  }
}

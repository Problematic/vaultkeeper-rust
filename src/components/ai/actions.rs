use super::considerations::*;
use super::{Decision, Need, Needs, PointOfInterest};
use crate::components::{Name, Perception, Position};
use ordered_float::NotNan;
use specs::{prelude::*, Component};

#[derive(Debug, Clone, Copy)]
pub enum AIInterest {
  Entity(Entity),
  Position(Position),
  POI(Need),
}

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
}

pub struct AIContext<'a> {
  pub agent: AICharacterData<'a>,
  pub entities: &'a Entities<'a>,
  pub positions: &'a ReadStorage<'a, Position>,
  pub points_of_interest: &'a ReadStorage<'a, PointOfInterest>,
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
          action: AIAction::Goto(AIInterest::POI(Need::Social)),
          considerations: vec![
            Box::new(NeedConsideration { need: Need::Social }),
            Box::new(DistanceToInterestConsideration { need: Need::Social }),
          ],
        },
        Decision {
          name: String::from("Go to food POI"),
          weight: 1.0,
          action: AIAction::Goto(AIInterest::POI(Need::Hunger)),
          considerations: vec![
            Box::new(NeedConsideration { need: Need::Hunger }),
            Box::new(DistanceToInterestConsideration { need: Need::Hunger }),
          ],
        },
      ],
    }
  }
}

impl AvailableActions {
  #[must_use]
  pub fn evaluate(&self, context: &AIContext) -> Option<(AIAction, f32)> {
    self
      .decisions
      .iter()
      .map(|d| (**d, d.score(context)))
      .max_by_key(|r| NotNan::from(r.1))
  }
}

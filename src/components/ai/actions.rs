use super::considerations::*;
use super::{AIContext, Decision, Need};
use crate::components::{Position, RadialZone};
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

impl AIAction {
  pub fn select(self, context: &mut AIContext) {
    use AIAction::*;

    match self {
      Goto(interest) => match interest {
        AIInterest::BlackboardTarget => {
          if let Some(pos) = context.get_entity_pos(context.blackboard.target) {
            if let Some(poi) = context
              .blackboard
              .target
              .and_then(|e| context.points_of_interest.get(e))
            {
              context.navigation.goal = Some(Box::new(RadialZone(pos, poi.range)));
            } else {
              context.navigation.goal = Some(Box::new(pos));
            }
          } else {
            log::warn!(
              "Couldn't find a position for {}'s target ({:?})",
              context.agent.name,
              context.blackboard.target
            );
          }
        }
        _ => unimplemented!(),
      },
      _ => unimplemented!(),
    }
  }

  pub fn is_done(self, context: &AIContext) -> bool {
    use AIAction::*;

    match self {
      Goto(interest) => match interest {
        AIInterest::BlackboardTarget => {
          if let Some(pos) = context.get_entity_pos(context.blackboard.target) {
            context.navigation.at_goal(pos)
          } else {
            true
          }
        }
        _ => unimplemented!(),
      },
      _ => unimplemented!(),
    }
  }
}

#[derive(Component, Debug)]
pub struct CurrentAction(pub AIAction);

use std::ops;

impl ops::Deref for CurrentAction {
  type Target = AIAction;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

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

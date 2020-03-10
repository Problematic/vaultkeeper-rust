use crate::components::ai::{
  AIAction, AIContext, AIInterest, AgentInfo, AvailableActions, CurrentAction, Need, Needs,
  PointOfInterest,
};
use crate::components::{Perception, Position};
use crate::utils;
use ordered_float::NotNan;
use specs::prelude::*;

#[derive(Default)]
pub struct ActionSelectionSystem {
  pending: Vec<(Entity, AIAction)>,
}

type SystemData<'a> = (
  Entities<'a>,
  ReadStorage<'a, Position>,
  ReadStorage<'a, Perception>,
  ReadStorage<'a, Needs>,
  ReadStorage<'a, AvailableActions>,
  ReadStorage<'a, PointOfInterest>,
  WriteStorage<'a, CurrentAction>,
);

impl<'a> System<'a> for ActionSelectionSystem {
  type SystemData = SystemData<'a>;

  #[allow(clippy::cast_precision_loss)]
  fn run(
    &mut self,
    (entities, positions, perceptions, needs, available_actions, pois, mut current_actions): Self::SystemData,
  ) {
    self.pending.clear();

    for (entity, position, perception, needs, actions, _) in (
      &entities,
      &positions,
      &perceptions,
      &needs,
      &available_actions,
      !&current_actions,
    )
      .join()
    {
      let context = AIContext {
        agent: AgentInfo {
          position,
          needs,
          perception,
        },
        distance_to: &|pos| utils::geom::chebyshev_dist(*position, pos),
        distance_to_interest: &|interest| match interest {
          AIInterest::Entity(e) => {
            if let Some(pos) = positions.get(e) {
              Some(utils::geom::chebyshev_dist(*position, *pos) as f32)
            } else {
              None
            }
          }
          AIInterest::POI(need) => (&positions, &pois)
            .join()
            .map(|(pos, poi)| {
              let dist = utils::geom::chebyshev_dist(*position, *pos) as f32;

              if poi.need != need || (dist > perception.range as f32 && !poi.is_global) {
                NotNan::from(0.0)
              } else {
                NotNan::from(dist / perception.range as f32)
              }
            })
            .min()
            .map(|v| v.into()),
          AIInterest::Position(pos) => Some(utils::geom::chebyshev_dist(*position, pos) as f32),
        },
      };
      if let Some((action, score)) = actions.evaluate(&context) {
        log::debug!("Selected action {:?} (score: {})", action, score);
        self.pending.push((entity, action));
      }
    }

    for (entity, action) in &self.pending {
      current_actions
        .insert(*entity, CurrentAction(*action))
        .unwrap();
    }
  }
}

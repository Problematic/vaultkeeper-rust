use crate::components::ai::{
  AIAction, AICharacterData, AIContext, AIInterest, AvailableActions, CurrentAction, Need, Needs,
  PointOfInterest,
};
use crate::components::{Name, Perception, Position};
use crate::utils;
use specs::prelude::*;

#[derive(Default)]
pub struct ActionSelectionSystem {}

type SystemData<'s> = (
  Entities<'s>,
  ReadStorage<'s, Position>,
  ReadStorage<'s, Perception>,
  ReadStorage<'s, Name>,
  ReadStorage<'s, AvailableActions>,
  ReadStorage<'s, PointOfInterest>,
  ReadStorage<'s, CurrentAction>,
  Read<'s, LazyUpdate>,
);

impl<'a> System<'a> for ActionSelectionSystem {
  type SystemData = SystemData<'a>;

  #[allow(clippy::cast_precision_loss)]
  fn run(
    &mut self,
    (
      entities,
      positions,
      perceptions,
      names,
      available_actions,
      points_of_interest,
      current_actions,
      lazy,
    ): Self::SystemData,
  ) {
    for (entity, name, position, perception, actions, _) in (
      &entities,
      &names,
      &positions,
      &perceptions,
      &available_actions,
      !&current_actions,
    )
      .join()
    {
      let context = AIContext {
        agent: AICharacterData {
          entity,
          name,
          position,
          perception,
        },
        entities: &entities,
        positions: &positions,
        points_of_interest: &points_of_interest,
      };

      if let Some((action, score)) = actions.evaluate(&context) {
        log::debug!("Selected action {:?} (score: {})", action, score);
        lazy.insert(entity, CurrentAction(action));
      }
    }
  }
}

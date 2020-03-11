use crate::components::ai::{
  AICharacterData, AIContext, AvailableActions, Blackboard, CurrentAction, Needs, PointOfInterest,
};
use crate::components::{Name, Perception, Position};
use specs::prelude::*;
use std::time::Instant;

#[derive(Default)]
pub struct ActionSelectionSystem {}

type SystemData<'s> = (
  Entities<'s>,
  ReadStorage<'s, Position>,
  ReadStorage<'s, Perception>,
  ReadStorage<'s, Name>,
  ReadStorage<'s, Needs>,
  ReadStorage<'s, AvailableActions>,
  ReadStorage<'s, PointOfInterest>,
  ReadStorage<'s, CurrentAction>,
  WriteStorage<'s, Blackboard>,
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
      needs_coll,
      available_actions,
      points_of_interest,
      current_actions,
      mut blackboards,
      lazy,
    ): Self::SystemData,
  ) {
    for (entity, name, position, needs, perception, actions, blackboard, _) in (
      &entities,
      &names,
      &positions,
      &needs_coll,
      &perceptions,
      &available_actions,
      &mut blackboards,
      !&current_actions,
    )
      .join()
    {
      let mut context = AIContext {
        agent: AICharacterData {
          entity,
          name,
          position,
          perception,
          needs,
        },
        entities: &entities,
        positions: &positions,
        points_of_interest: &points_of_interest,
        blackboard,
      };

      let start = Instant::now();

      if let Some((action, decision_name, score)) = actions.evaluate(&mut context) {
        log::debug!(
          "{} selected '{}' (score: {}, elapsed: {}\u{3bc}s)",
          name,
          decision_name,
          score,
          start.elapsed().as_micros()
        );
        lazy.insert(entity, CurrentAction(action));
      } else {
        log::warn!("No viable actions found for {}", name);
      }
    }
  }
}

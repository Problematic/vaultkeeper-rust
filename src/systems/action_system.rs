use crate::resources::DeltaTime;
use components::{
  AICharacterData, AIContext, AvailableActions, Blackboard, CurrentAction, Name, Navigation, Needs,
  Perception, PointOfInterest, Position,
};
use specs::prelude::*;
use std::time::Instant;
use utils;

#[derive(Default)]
pub struct ActionSystem {}

type SystemData<'s> = (
  Entities<'s>,
  ReadExpect<'s, DeltaTime>,
  ReadStorage<'s, Position>,
  ReadStorage<'s, Perception>,
  ReadStorage<'s, Name>,
  ReadStorage<'s, Needs>,
  ReadStorage<'s, AvailableActions>,
  ReadStorage<'s, PointOfInterest>,
  ReadStorage<'s, CurrentAction>,
  WriteStorage<'s, Blackboard>,
  WriteStorage<'s, Navigation>,
  Read<'s, LazyUpdate>,
);

impl<'a> System<'a> for ActionSystem {
  type SystemData = SystemData<'a>;

  #[allow(clippy::cast_precision_loss)]
  fn run(
    &mut self,
    (
      entities,
      delta_time,
      positions,
      perceptions,
      names,
      needs_coll,
      available_actions,
      points_of_interest,
      current_actions,
      mut blackboards,
      mut navigations,
      lazy,
    ): Self::SystemData,
  ) {
    for (
      entity,
      name,
      position,
      needs,
      perception,
      actions,
      blackboard,
      navigation,
      current_action,
    ) in (
      &entities,
      &names,
      &positions,
      &needs_coll,
      &perceptions,
      &available_actions,
      &mut blackboards,
      &mut navigations,
      current_actions.maybe(),
    )
      .join()
    {
      for cooldown in blackboard.cooldowns.values_mut() {
        *cooldown = cooldown
          .checked_sub(**delta_time)
          .unwrap_or(utils::ZERO_DURATION);
      }

      let mut context = AIContext {
        dt: **delta_time,
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
        navigation,
      };

      if let Some(CurrentAction(action)) = current_action {
        if action.is_done(&context) {
          lazy.remove::<CurrentAction>(entity);
        }
      } else {
        let start = Instant::now();

        if let Some((decision, score)) = actions.evaluate(&mut context) {
          log::debug!(
            "{} selected '{}' (score: {}, elapsed: {}\u{3bc}s)",
            name,
            decision.name,
            score,
            start.elapsed().as_micros()
          );
          decision.action.select(&mut context);
          lazy.insert(entity, CurrentAction(decision.action));
        } else {
          log::warn!("No viable actions found for {}", name);
        }
      }
    }
  }
}

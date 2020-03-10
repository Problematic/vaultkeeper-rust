use crate::components::ai::{
  AIAction, AICharacterData, AIContext, AIInterest, AvailableActions, CurrentAction, Need, Needs,
  PointOfInterest, WithDistance,
};
use crate::components::{Character, Name, Perception, Position};
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
  ReadStorage<'s, Character>,
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
      pois,
      current_actions,
      characters,
      lazy,
    ): Self::SystemData,
  ) {
    for (entity, name, position, perception, character, actions, _) in (
      &entities,
      &names,
      &positions,
      &perceptions,
      &characters,
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
        },
        characters: (&entities, &names, &positions, &characters)
          .join()
          .filter_map(|(e, name, pos, _)| {
            let dist = utils::geom::chebyshev_dist(*position, *pos);

            if dist <= perception.range && e != entity {
              Some(WithDistance(
                AICharacterData {
                  entity: e,
                  name,
                  position: pos,
                },
                dist,
              ))
            } else {
              None
            }
          })
          .collect(),
        points_of_interest: (&pois, &positions)
          .join()
          .filter_map(|(poi, pos)| {
            let dist = utils::geom::chebyshev_dist(*position, *pos);
            if poi.is_global || dist <= perception.range {
              Some(WithDistance(poi, dist))
            } else {
              None
            }
          })
          .collect(),
      };

      if let Some((action, score)) = actions.evaluate(&context) {
        log::debug!("Selected action {:?} (score: {})", action, score);
        lazy.insert(entity, CurrentAction(action));
      }
    }
  }
}

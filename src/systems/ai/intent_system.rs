use crate::components::{
  tags::{Monster, Player},
  Awareness, Intent,
};
use legion::prelude::*;

pub fn build_intent_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("ai_intent")
    .read_component::<Player>()
    .with_query(<Read<Awareness>>::query().filter(tag::<Monster>() & !component::<Intent>()))
    .build(|cmd, world, _, query| {
      for (entity, awareness) in query.iter_entities(&world) {
        if let Some(player) = awareness
          .visible_entities
          .iter()
          .find(|e| world.get_tag::<Player>(**e).is_some())
        {
          cmd.add_component(entity, Intent::Attack(*player));
        }
      }
    })
}

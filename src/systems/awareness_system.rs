use crate::components::{
  effects::{Effect, TimedEffect},
  tags::Player,
  Appearance, Awareness, Lifetime, Memory, Name, Position, Viewshed,
};
use crate::resources::WorldMap;
use bracket_lib::prelude::*;
use legion::prelude::*;
use std::time::Duration;

pub fn build_awareness_system() -> Box<dyn Schedulable> {
  let encounter_effect = TimedEffect::new(
    Appearance::new('!', LIGHTGREY, BLACK),
    Lifetime::GameTime(Duration::from_millis(500)),
  );

  SystemBuilder::new("awareness")
    .read_resource::<WorldMap>()
    .read_component::<Player>()
    .read_component::<Name>()
    .with_query(<(
      Read<Viewshed>,
      Read<Position>,
      Write<Awareness>,
      Write<Memory>,
    )>::query())
    .build(move |cmd, mut world, map, query| {
      for (entity, (viewshed, position, mut awareness, mut memory)) in
        query.iter_entities_mut(&mut world)
      {
        let is_player = world.get_component::<Player>(entity).is_some();

        let encounters = viewshed
          .visible_tiles
          .iter()
          .filter_map(|pos| map[*pos].occupant.map(|e| (e, pos)));

        awareness.visible_entities.clear();

        for (e, pos) in encounters {
          if e == entity {
            continue;
          }

          awareness.visible_entities.insert(e);

          if memory.encounters.insert(e, *pos).is_none() {
            if !is_player && world.get_tag::<Player>(e).is_some() {
              encounter_effect.spawn(cmd, *position);
            } else {
            }
          }
        }
      }
    })
}

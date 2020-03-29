use crate::components::{
  actions::AttackAction,
  effects::{Effect, TimedEffect},
  markers::HasAction,
  Appearance, Health, Lifetime, Position,
};
use crate::resources::WorldMap;
use bracket_lib::prelude::*;
use legion::prelude::*;
use std::time::Duration;

pub fn build_attack_system() -> Box<dyn Schedulable> {
  let defeated_effect = TimedEffect::new(
    Appearance::new('x', GREY, BLACK),
    Lifetime::GameTime(Duration::from_millis(300)),
  );

  let hit_effect = TimedEffect::new(
    Appearance::new('☼', RED, BLACK),
    Lifetime::GameTime(Duration::from_millis(250)),
  );

  SystemBuilder::new("attack")
    .write_resource::<WorldMap>()
    .write_component::<Health>()
    .with_query(<(Read<Position>, Read<AttackAction>)>::query())
    .build(move |cmd, world, map, query| {
      for (entity, (pos, action)) in query.iter_entities(&world) {
        let dest = *pos + action.direction.as_delta_pos();
        if let Some(target) = map[dest].occupant {
          if let Some(mut health) = world.get_component_mut::<Health>(target) {
            // TODO: replace with actual combat stats
            *health -= 10;

            if health.current == 0 {
              // TODO: have a cleanup system to manage this?
              {
                cmd.delete(target);
                map[dest].occupant = None;

                defeated_effect.spawn(cmd, dest);
              }
            } else {
              hit_effect.spawn(cmd, dest);
            }
          } else {
            log::error!(
              "Found an occupant who didn't have health at {:?}: {:?}",
              dest,
              target
            );
            map[dest].occupant = None;
          }
        } else {
          log::warn!("Whiff! (Target wasn't present for some reason?)");
        }

        cmd.remove_component::<AttackAction>(entity);
        cmd.remove_component::<HasAction>(entity);
      }
    })
}

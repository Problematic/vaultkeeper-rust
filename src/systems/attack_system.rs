use crate::components::{
  actions::AttackAction, tags::Effect, Appearance, Health, Lifetime, Position,
};
use crate::resources::WorldMap;
use bracket_lib::prelude::*;
use legion::prelude::*;
use std::time::Duration;

pub fn build_attack_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("attack")
    .write_resource::<WorldMap>()
    .write_component::<Health>()
    .with_query(<(Read<Position>, Read<AttackAction>)>::query())
    .build(|cmd, world, map, query| {
      for (entity, (pos, action)) in query.iter_entities(&world) {
        let dest = *pos + action.direction.as_delta_pos();
        if let Some(target) = map[dest].occupant {
          let mut health = world.get_component_mut::<Health>(target).unwrap();

          // TODO: replace with actual combat stats
          *health -= 10;

          if health.current == 0 {
            // TODO: display log messages to player
            log::info!("Target slain!");

            // TODO: have a cleanup system to manage this?
            {
              cmd.delete(target);
              map[dest].occupant = None;

              cmd
                .start_entity()
                .with_tag((Effect,))
                .with_component(dest)
                .with_component(Appearance::new('x', GREY, BLACK))
                .with_component(Lifetime::GameTime(Duration::from_millis(300)))
                .build();
            }
          } else {
            log::info!("Whack!");

            cmd
              .start_entity()
              .with_tag((Effect,))
              .with_component(dest)
              .with_component(Appearance::new('☼', RED, BLACK))
              .with_component(Lifetime::GameTime(Duration::from_millis(250)))
              .build();
          }
        } else {
          log::warn!("Whiff! (Target wasn't present for some reason?)");
        }

        cmd.remove_component::<AttackAction>(entity);
      }
    })
}

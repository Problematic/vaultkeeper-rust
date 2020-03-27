use crate::components::{
  tags::{Mobile, Player},
  Energy, Speed,
};
use legion::prelude::*;

pub fn build_energy_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("energy")
    .with_query(<(Read<Speed>, Write<Energy>)>::query().filter(tag::<Player>()))
    .with_query(<(Read<Speed>, Write<Energy>)>::query().filter(tag::<Mobile>() & !tag::<Player>()))
    .build(|_, mut world, _, queries| {
      for (speed, mut energy) in queries.0.iter_mut(&mut world) {
        energy.gain(speed.0);

        if energy.can_act() {
          // Player is ready to act, so we want to "block" NPC
          // actions until they do so
          return;
        }
      }

      for (speed, mut energy) in queries.1.iter_mut(&mut world) {
        energy.gain(speed.0);
      }
    })
}

use crate::components::Needs;
use crate::resources::DeltaTime;
use specs::prelude::*;

#[derive(Default)]
pub struct NeedDecaySystem;

impl<'a> System<'a> for NeedDecaySystem {
  type SystemData = (ReadExpect<'a, DeltaTime>, WriteStorage<'a, Needs>);

  fn run(&mut self, (delta_time, mut needs): Self::SystemData) {
    let DeltaTime(dt) = *delta_time;

    for (needs,) in (&mut needs,).join() {
      for val in needs.0.values_mut() {
        // TODO: adjust this with agent personality stats
        *val -= dt.as_secs_f32();
      }
    }
  }
}

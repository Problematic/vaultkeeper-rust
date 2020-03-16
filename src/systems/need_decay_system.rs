use crate::resources::DeltaTime;
use components::*;
use legion::prelude::*;

pub fn build_need_decay_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("need_decay")
    .read_resource::<DeltaTime>()
    .with_query(<(Write<Needs>,)>::query())
    .build(|_, mut world, delta_time, query| {
      let DeltaTime(dt) = **delta_time;

      for (mut needs,) in query.iter_mut(&mut world) {
        for val in needs.0.values_mut() {
          // TODO: adjust this with agent personality stats
          *val -= dt.as_secs_f32();
        }
      }
    })
}

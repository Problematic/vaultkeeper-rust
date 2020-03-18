use components::*;
use legion::prelude::*;
use vaultkeeper_shared::Time;

pub fn build_need_decay_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("need_decay")
    .read_resource::<Time>()
    .with_query(<(Write<Needs>,)>::query())
    .build(|_, mut world, time, query| {
      let dt = time.dt.as_secs_f32();

      for (mut needs,) in query.iter_mut(&mut world) {
        for val in needs.0.values_mut() {
          // TODO: adjust this with agent personality stats
          *val -= dt;
        }
      }
    })
}

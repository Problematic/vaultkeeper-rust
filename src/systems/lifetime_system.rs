use crate::components::Lifetime;
use crate::resources::GameTime;
use legion::prelude::*;

pub fn build_lifetime_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("lifetime")
    .read_resource::<GameTime>()
    .with_query(<Write<Lifetime>>::query())
    .build(|cmd, mut world, time, query| {
      for (entity, mut lifetime) in query.iter_entities_mut(&mut world) {
        use Lifetime::*;

        match *lifetime {
          Frames(count) => {
            if let Some(remaining) = count.checked_sub(1) {
              *lifetime = Frames(remaining);
            } else {
              cmd.delete(entity);
            }
          }
          GameTime(duration) => {
            if let Some(remaining) = duration.checked_sub(time.dt) {
              *lifetime = GameTime(remaining);
            } else {
              cmd.delete(entity);
            }
          }
        }
      }
    })
}

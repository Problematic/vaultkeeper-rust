use components::utils::chebyshev_dist;
use components::*;
use legion::prelude::*;

pub fn build_visibility_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("visibility")
    .with_query(<(Read<Position>, Read<Perception>, Write<Viewshed>)>::query())
    .with_query(<Read<Position>>::query())
    .build(|_, mut world, _, queries| {
      for (entity, (position, perception, mut viewshed)) in queries.0.iter_entities_mut(&mut world)
      {
        viewshed.visible_entities.clear();

        for (e, pos) in queries.1.iter_entities(&world) {
          if e == entity {
            continue;
          }

          if chebyshev_dist(*position, *pos) <= perception.range {
            viewshed.visible_entities.push(e);
          }
        }
      }
    })
}

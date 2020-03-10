use crate::components::{Perception, Position, Viewshed};
use crate::utils;
use specs::prelude::*;

#[derive(Default)]
pub struct VisibilitySystem {
  missing_viewsheds: Vec<Entity>,
}

impl<'a> System<'a> for VisibilitySystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Perception>,
    WriteStorage<'a, Viewshed>,
  );

  fn run(&mut self, (entities, positions, perceptions, mut viewsheds): Self::SystemData) {
    self.missing_viewsheds.clear();
    for (entity, _, _) in (&entities, &perceptions, !&viewsheds).join() {
      self.missing_viewsheds.push(entity);
    }
    for entity in self.missing_viewsheds.drain(..) {
      viewsheds.insert(entity, Viewshed::default()).unwrap();
    }

    for (entity, position, perception, viewshed) in
      (&entities, &positions, &perceptions, &mut viewsheds).join()
    {
      viewshed.visible_entities.clear();

      for (e, pos) in (&entities, &positions).join() {
        if entity == e {
          continue;
        }

        if utils::geom::chebyshev_dist(*position, *pos) <= perception.range {
          viewshed.visible_entities.push(e);
        }
      }
    }
  }
}

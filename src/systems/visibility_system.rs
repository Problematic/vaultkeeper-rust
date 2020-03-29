use crate::components::{tags::Player, Position, Viewshed};
use crate::resources::WorldMap;
use legion::prelude::*;

pub fn build_visibility_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("visibility")
    .write_resource::<WorldMap>()
    .with_query(<(Read<Position>, Write<Viewshed>)>::query().filter(changed::<Position>()))
    .build(|_, mut world, map, query| {
      for (entity, (pos, mut viewshed)) in query.iter_entities_mut(&mut world) {
        viewshed.update(map, *pos);

        if world.get_tag::<Player>(entity).is_some() {
          viewshed
            .visible_tiles
            .iter()
            .for_each(|pos| map[*pos].is_revealed = true);
        }
      }
    })
}

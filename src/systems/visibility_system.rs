use crate::components::{Position, Viewshed};
use crate::resources::WorldMap;
use bracket_lib::prelude::*;
use legion::prelude::*;

pub fn build_visibility_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("visibility")
    .write_resource::<WorldMap>()
    .with_query(<(Read<Position>, Write<Viewshed>)>::query().filter(changed::<Position>()))
    .build(|_, mut world, map, query| {
      for (pos, mut viewshed) in query.iter_mut(&mut world) {
        viewshed.visible_tiles.clear();

        let visible_tiles = field_of_view_set(*pos, viewshed.range, &**map);

        viewshed.visible_tiles.extend(visible_tiles);
        viewshed.visible_tiles.retain(|p| map.in_bounds(*p));

        viewshed
          .visible_tiles
          .iter()
          .for_each(|pos| map[*pos].is_revealed = true);
      }
    })
}

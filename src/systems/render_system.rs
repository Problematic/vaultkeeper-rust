use crate::components::{
  tags::{Effect, Player},
  Appearance, Position, Viewshed,
};
use crate::resources::WorldMap;
use bracket_lib::prelude::*;
use legion::prelude::*;
use std::collections::HashSet;

pub fn build_render_system() -> Box<dyn Schedulable> {
  let mut visible_tiles = HashSet::<Position>::with_capacity(256);

  SystemBuilder::new("render")
    .read_resource::<WorldMap>()
    .with_query(<Read<Viewshed>>::query().filter(tag::<Player>()))
    .with_query(<(Read<Position>, Read<Appearance>)>::query().filter(!tag::<Effect>()))
    .with_query(<(Read<Position>, Read<Appearance>)>::query().filter(tag::<Effect>()))
    .build(move |_, world, map, queries| {
      visible_tiles.clear();

      let mut batch = DrawBatch::new();
      batch.cls();

      map.render(&mut batch);

      for viewshed in queries.0.iter(&world) {
        visible_tiles.extend(viewshed.visible_tiles.iter());
        viewshed.visible_tiles.iter().for_each(|pos| {
          let Appearance { glyph, colors } = map[*pos].appearance();

          batch.set(*pos, colors, glyph);
        });
      }

      for (position, appearance) in queries.1.iter(&world) {
        if !visible_tiles.contains(&position) {
          continue;
        }

        let Appearance { glyph, colors } = *appearance;

        batch.set(*position, colors, glyph);
      }

      for (position, appearance) in queries.2.iter(&world) {
        if !visible_tiles.contains(&position) {
          continue;
        }

        let Appearance { glyph, colors } = *appearance;

        batch.set(*position, colors, glyph);
      }

      batch.submit(0).unwrap();
    })
}

#![warn(clippy::pedantic)]

mod components;
mod resources;
mod state;
mod systems;

use crate::components::*;
use crate::state::State;
use bracket_lib::prelude::*;
use specs::prelude::*;

fn main() {
  let context = BTermBuilder::simple(80, 60)
    .with_title("Vaultkeeper")
    .with_tile_dimensions(16, 16)
    .build();

  let mut state = State::new();

  state.world.register::<Point>();
  state.world.register::<Renderable>();

  state
    .world
    .create_entity()
    .with(Point::new(10, 10))
    .with(Renderable {
      glyph: to_cp437('☺'),
      fg: RGB::named(WHITE),
      bg: RGB::named(BLACK),
    })
    .build();

  main_loop(context, state);
}

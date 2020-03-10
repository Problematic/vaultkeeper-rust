#![warn(clippy::pedantic)]

mod components;
mod resources;
mod state;
mod systems;
mod utils;

use crate::components::ai::*;
use crate::components::*;
use crate::state::State;
use crate::systems::ai::*;
use crate::systems::*;
use bracket_lib::prelude::*;
use rand::Rng;
use specs::prelude::*;

pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 60;

fn main() {
  pretty_env_logger::init_timed();
  let mut rng = rand::thread_rng();

  let context = BTermBuilder::simple(WINDOW_WIDTH, WINDOW_HEIGHT)
    .with_title("Vaultkeeper")
    .with_tile_dimensions(16, 16)
    .build();

  let db = DispatcherBuilder::new()
    .with(VisibilitySystem::default(), "visibility", &[])
    .with(NeedDecaySystem::default(), "need_decay", &[])
    .with(
      ActionSelectionSystem::default(),
      "action_selection",
      &["need_decay"],
    )
    .with(
      PathfinderSystem::default(),
      "pathfinder",
      &["action_selection"],
    )
    .with_barrier();

  db.print_par_seq();

  let mut dispatcher = db.build();
  let mut world = World::new();
  dispatcher.setup(&mut world);

  world.register::<Renderable>();
  world.register::<Name>();

  let mut state = State::new(world, dispatcher);

  state
    .world
    .create_entity()
    .with(Name::from("Watercooler"))
    .with(Position::new(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2))
    .with(Renderable {
      glyph: to_cp437('#'),
      fg: RGB::named(AQUA),
      bg: RGB::named(BLACK),
    })
    .with(PointOfInterest {
      is_global: true,
      need: Need::Social,
      range: 4,
    })
    .build();

  state
    .world
    .create_entity()
    .with(Name::from("Cake"))
    .with(Position::new(70, 45))
    .with(Renderable {
      glyph: to_cp437('O'),
      fg: RGB::named(PINK),
      bg: RGB::named(BLACK),
    })
    .with(PointOfInterest {
      is_global: false,
      need: Need::Hunger,
      range: 1,
    })
    .build();

  for (idx, pos) in [Position::new(10, 10), Position::new(70, 50)]
    .iter()
    .enumerate()
  {
    state
      .world
      .create_entity()
      .with(Name(format!("Vaultizen #{:0>3}", idx + 1)))
      .with(*pos)
      .with(Renderable {
        glyph: to_cp437('☺'),
        fg: RGB::named(WHITE),
        bg: RGB::named(BLACK),
      })
      .with(Perception { range: 5 })
      .with(Needs::from(vec![
        (Need::Hunger, rng.gen_range(35.0, 75.0)),
        (Need::Social, rng.gen_range(35.0, 75.0)),
      ]))
      .with(Navigation::default())
      .with(AvailableActions::default())
      .build();
  }

  main_loop(context, state);
}

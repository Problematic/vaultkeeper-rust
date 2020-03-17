#![warn(clippy::pedantic)]

mod state;
mod systems;

use crate::state::State;
use crate::systems::*;
use bracket_lib::prelude::*;
use components::*;
use legion::prelude::*;
use rand::Rng;

pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 60;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy)]
pub enum RunState {
  Running,
  Paused,
}

impl std::ops::Not for RunState {
  type Output = RunState;

  fn not(self) -> Self {
    match self {
      RunState::Running => RunState::Paused,
      RunState::Paused => RunState::Running,
    }
  }
}

#[allow(clippy::too_many_lines)]
fn main() {
  pretty_env_logger::init_timed();
  let mut rng = rand::thread_rng();

  let context = BTermBuilder::simple(WINDOW_WIDTH, WINDOW_HEIGHT)
    .with_title("Vaultkeeper")
    .with_tile_dimensions(16, 16)
    .with_fps_cap(5.0) // TODO: limit agent tick rate without FPS cap
    .build();

  let universe = Universe::new();
  let world = universe.create_world();

  let sb = Schedule::builder()
    .add_system(build_need_decay_system())
    .add_system(build_visibility_system())
    .add_system(build_pathfinder_system())
    .add_system(build_movement_system())
    .add_system(ai::interactables::cake::build_system())
    .flush();

  let mut state = State {
    run_state: RunState::Running,
    world,
    resources: Resources::default(),
    schedule: sb.build(),
  };

  state.world.insert(
    (),
    vec![(
      Name::new("Watercooler"),
      Position::new(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2),
      Renderable {
        glyph: to_cp437('#'),
        fg: RGB::named(AQUA),
        bg: RGB::named(BLACK),
      },
      PointOfInterest {
        is_global: true,
        need: Need::Social,
        range: 4,
      },
    )],
  );

  state.world.insert(
    (),
    vec![(
      Name::new("Cake"),
      Position::new(70, 45),
      Renderable {
        glyph: to_cp437('O'),
        fg: RGB::named(PINK),
        bg: RGB::named(BLACK),
      },
      PointOfInterest {
        is_global: false,
        need: Need::Hunger,
        range: 1,
      },
    )],
  );

  state.world.insert(
    (Character,),
    vec![Position::new(10, 10), Position::new(70, 50)]
      .into_iter()
      .enumerate()
      .map(|(idx, pos)| {
        (
          Name::new(format!("Vaultizen #{:0>3}", idx + 1)),
          pos,
          Renderable {
            glyph: to_cp437('☺'),
            fg: RGB::named(WHITE),
            bg: RGB::named(BLACK),
          },
          Perception { range: 5 },
          Needs::from(vec![
            (Need::Hunger, rng.gen_range(35.0, 75.0)),
            (Need::Social, rng.gen_range(35.0, 75.0)),
          ]),
          Navigation::default(),
          Viewshed::default(),
        )
      }),
  );

  main_loop(context, state);
}

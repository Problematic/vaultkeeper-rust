#![warn(clippy::pedantic)]

mod game;
mod states;
mod systems;

use crate::game::*;
use crate::states::*;
use crate::systems::*;
use bracket_lib::prelude::*;
use legion::prelude::*;
use resources::*;

pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 60;

#[derive(Debug, PartialEq)]
pub enum GameMode {
  Sim,
  Delve,
}

impl std::str::FromStr for GameMode {
  type Err = ();

  fn from_str(s: &str) -> Result<GameMode, Self::Err> {
    match s {
      "delve" => Ok(GameMode::Delve),
      "sim" | "simulation" => Ok(GameMode::Sim),
      _ => Err(()),
    }
  }
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  pretty_env_logger::init_timed();

  let args: Vec<String> = std::env::args().collect();
  let mode = if args.len() >= 2 {
    args[1]
      .parse::<GameMode>()
      .expect("Unrecognized mode; expected one of `sim | delve`")
  } else {
    GameMode::Sim
  };

  let mut context = BTermBuilder::simple(WINDOW_WIDTH, WINDOW_HEIGHT)?
    .with_title("Vaultkeeper")
    .with_tile_dimensions(16, 16)
    .build()?;

  let universe = Universe::new();
  let world = universe.create_world();

  let sb = Schedule::builder()
    .add_system(build_need_decay_system())
    .add_system(build_visibility_system())
    .add_system(build_pathfinder_system())
    .add_system(build_movement_system())
    .flush();

  let sb = ai::systems::register_systems(sb);

  let mut resources = Resources::default();

  let state: Box<dyn VaultkeeperState> = match mode {
    GameMode::Delve => Box::new(DelveState::default()),
    GameMode::Sim => Box::new(SimState::default()),
  };

  let mut game = Game {
    context: WorldContext { world, resources },
    schedule: sb.build(),
    state_stack: vec![state],
  };

  game.init(&mut context);

  main_loop(context, game)
}

#![warn(clippy::all)]

mod components;
mod game;
mod resources;
mod systems;
mod utils;

use crate::components::Input;
use crate::game::Game;
use crate::resources::{GameTime, Keymap};
use bracket_lib::prelude::*;
use legion::prelude::*;
use rand::{rngs::StdRng, SeedableRng};

pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 60;

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  pretty_env_logger::init_timed();

  // TODO: load this from a command line argument / override file
  let keymap: Keymap<Input> = serde_json::from_str(include_str!("../resources/keybindings.json"))?;

  let context = BTermBuilder::simple(WINDOW_WIDTH, WINDOW_HEIGHT)?
    .with_title("Vaultkeeper")
    .with_tile_dimensions(16, 16)
    // .with_advanced_input(true)
    .build()?;

  let universe = Universe::new();
  let world = universe.create_world();

  let mut resources = Resources::default();
  resources.insert(keymap);
  resources.insert(GameTime::default());
  resources.insert(StdRng::from_rng(rand::thread_rng())?);

  let mut game = Game {
    world,
    resources,
    schedule: None,
  };

  game.init();

  main_loop(context, game)
}

#![warn(clippy::pedantic)]

mod game;
mod systems;

use crate::game::*;
use crate::systems::*;
use bracket_lib::prelude::*;
use legion::prelude::*;
use vaultkeeper_delve;
use vaultkeeper_shared::map as vk_map;
use vaultkeeper_shared::ui::Keybindings;
use vaultkeeper_shared::*;

pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 60;

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  pretty_env_logger::init_timed();

  let context = BTermBuilder::simple(WINDOW_WIDTH, WINDOW_HEIGHT)?
    .with_title("Vaultkeeper")
    .with_tile_dimensions(16, 16)
    // .with_advanced_input(true)
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
  resources.insert(Time::default());

  let map = vk_map::generators::BSPMapGenerator::<vk_map::MapTile>::new()
    .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
    .with_impassible_borders(true)
    .with_iterations(4)
    .with_partition_jitter(0.1)
    .with_room_size(0.75)
    .with_min_room_size((3, 3))
    .with_filled(true)
    .build();

  resources.insert(map);

  // TODO: load this from a command line argument / override file
  let keybindings: Keybindings =
    serde_json::from_str(include_str!("../resources/keybindings.json")).unwrap();

  let initial_state = vaultkeeper_delve::states::MainState::default();

  let mut game = Game {
    keybindings,
    world,
    resources,
    data: vaultkeeper_delve::StateData { action: None },
    schedule: sb.build(),
    state_machine: StateMachine::new(initial_state),
  };

  game.init();

  main_loop(context, game)
}

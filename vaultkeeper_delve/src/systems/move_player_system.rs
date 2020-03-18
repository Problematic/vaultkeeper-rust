use crate::components::*;
use components::*;
use legion::prelude::*;
use vaultkeeper_shared::{MoveDirection, PlayerInput};

pub fn build_move_player_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("move_player")
    .read_resource::<PlayerInput>()
    .with_query(<Write<Position>>::query().filter(component::<Player>()))
    .build(|_, mut world, player_input, query| {
      if let Some(direction) = &player_input.move_dir {
        for mut pos in query.iter_mut(&mut world) {
          *pos = *pos
            + match direction {
              MoveDirection::Up => Position::new(0, -1),
              MoveDirection::Down => Position::new(0, 1),
              MoveDirection::Left => Position::new(-1, 0),
              MoveDirection::Right => Position::new(1, 0),
            };
        }
      }
    })
}

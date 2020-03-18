use crate::components::*;
use crate::systems;
use bracket_lib::prelude::*;
use components::*;
use legion::prelude::*;
use vaultkeeper_shared::{State, Transition, WorldContext};

#[derive(Default)]
pub struct MainState {
  schedule: Option<Schedule>,
}

impl State for MainState {
  fn on_start(&mut self, _term: &mut BTerm, context: &mut WorldContext) {
    self.schedule = Some(
      Schedule::builder()
        .add_system(systems::build_move_player_system())
        .build(),
    );

    context.world.insert(
      (),
      vec![(
        Name::new("Player"),
        Player {},
        Position::new(25, 25),
        Renderable {
          glyph: to_cp437('@'),
          colors: ColorPair {
            fg: RGBA::named(WHITE),
            bg: RGBA::named(BLACK),
          },
        },
      )],
    );
  }

  fn update(&mut self, _term: &mut BTerm, context: &mut WorldContext) -> Transition {
    if let Some(schedule) = self.schedule.as_mut() {
      schedule.execute(&mut context.world, &mut context.resources);
    }

    Transition::None
  }
}

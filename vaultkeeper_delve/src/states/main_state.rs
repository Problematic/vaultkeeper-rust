use crate::{components::*, StateData};
use bracket_lib::prelude::*;
use components::*;
use legion::prelude::*;
use rand::seq::SliceRandom;
use vaultkeeper_shared::{
  map::MapTile, ui::Input as VKInput, State, StateContext, Transition, WorldMap,
};

#[derive(Default)]
pub struct MainState {
  schedule: Option<Schedule>,
}

impl State<StateData> for MainState {
  fn on_start(&mut self, context: &mut StateContext<StateData>) {
    let mut rng = rand::thread_rng();

    self.schedule = Some(Schedule::builder().build());

    let map = context.resources.get::<WorldMap<MapTile>>().unwrap();
    let start_pos = map.rooms.choose(&mut rng).unwrap().center();

    context.world.insert(
      (),
      vec![(
        Name::new("Player"),
        Player {},
        start_pos,
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

  fn update(&mut self, context: &mut StateContext<StateData>) -> Transition<StateData> {
    if let Some(schedule) = self.schedule.as_mut() {
      schedule.execute(&mut context.world, &mut context.resources);
    }

    Transition::None
  }

  fn handle_input(
    &mut self,
    _context: &mut StateContext<StateData>,
    input: VKInput,
  ) -> Transition<StateData> {
    dbg!(&input);

    Transition::None
  }
}

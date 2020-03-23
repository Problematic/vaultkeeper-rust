use bracket_lib::prelude::*;
use components::*;
use legion::prelude::*;
use std::time::Duration;
use vaultkeeper_shared::{
  map::MapTile, ui::Keybindings, Render, StateContext, StateMachine, Time, WorldMap,
};

pub struct Game<TData> {
  pub keybindings: Keybindings,
  pub schedule: Schedule,
  pub world: World,
  pub resources: Resources,
  pub data: TData,
  pub state_machine: StateMachine<'static, TData>,
}

impl<TData> Game<TData> {
  pub fn init(&mut self) {
    self.state_machine.start(StateContext::new(
      &mut self.world,
      &mut self.resources,
      &mut self.data,
    ));
  }
}

impl<TData: 'static> GameState for Game<TData> {
  fn tick(&mut self, term: &mut BTerm) {
    self
      .resources
      .get_mut::<Time>()
      .unwrap()
      .capture(Duration::from_secs_f32(term.frame_time_ms / 1000.0));

    term.cls();

    self.schedule.execute(&mut self.world, &mut self.resources);

    if let Some(input) = term.key.and_then(|key| self.keybindings.get(&key).copied()) {
      self.state_machine.handle_input(
        StateContext::new(&mut self.world, &mut self.resources, &mut self.data),
        input,
      );
    }

    self.state_machine.update(StateContext::new(
      &mut self.world,
      &mut self.resources,
      &mut self.data,
    ));

    let map = self.resources.get::<WorldMap<MapTile>>().unwrap();
    map.render(term);

    let query = <(Read<Position>, Read<Renderable>)>::query();
    for (pos, render) in query.iter(&self.world) {
      term.set(
        pos.x,
        pos.y,
        render.colors.fg,
        render.colors.bg,
        render.glyph,
      );
    }
  }
}

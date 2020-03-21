use bracket_lib::prelude::*;
use components::*;
use legion::prelude::*;
use std::time::Duration;
use vaultkeeper_shared::{
  map::MapTile, ui::Keybindings, Render, State, Time, Transition, WorldContext, WorldMap,
};

pub struct Game {
  pub keybindings: Keybindings,
  pub schedule: Schedule,
  pub context: WorldContext,
  pub state_stack: Vec<Box<dyn State>>,
}

impl Game {
  pub fn init(&mut self, term: &mut BTerm) {
    self
      .state_stack
      .last_mut()
      .unwrap()
      .on_start(term, &mut self.context);
  }
}

impl GameState for Game {
  fn tick(&mut self, term: &mut BTerm) {
    self
      .context
      .resources
      .get_mut::<Time>()
      .unwrap()
      .capture(Duration::from_secs_f32(term.frame_time_ms / 1000.0));

    term.cls();

    self
      .schedule
      .execute(&mut self.context.world, &mut self.context.resources);

    if let Some(mut active) = self.state_stack.pop() {
      if let Some(input) = term.key.and_then(|key| self.keybindings.get(&key).copied()) {
        // TODO: traverse the stack until the input is handled or we're done
        let _handled = active.handle_input(term, &mut self.context, input);
      }

      let transition = active.update(term, &mut self.context);

      match transition {
        Transition::Push(mut state) => {
          active.on_pause(term, &mut self.context);
          self.state_stack.push(active);
          state.on_start(term, &mut self.context);
          self.state_stack.push(state);
        }
        Transition::Switch(mut state) => {
          active.on_stop(term, &mut self.context);
          state.on_start(term, &mut self.context);
          self.state_stack.push(state);
        }
        Transition::Pop => {
          active.on_stop(term, &mut self.context);
          if let Some(next) = self.state_stack.last_mut() {
            next.on_resume(term, &mut self.context);
          }
        }
        Transition::None => {
          self.state_stack.push(active);
        }
        Transition::Quit => {
          term.quit();
          return;
        }
      }
    } else {
      panic!("No active state; make sure there's a fallback.");
    }

    let map = self.context.resources.get::<WorldMap<MapTile>>().unwrap();
    map.render(term);

    let query = <(Read<Position>, Read<Renderable>)>::query();
    for (pos, render) in query.iter(&self.context.world) {
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

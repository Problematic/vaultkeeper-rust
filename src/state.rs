use crate::resources::*;
use bracket_lib::prelude::*;
use components::*;
use specs::prelude::*;
use std::time::Duration;

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

pub struct State {
  pub run_state: RunState,
  pub world: World,
  pub dispatcher: Dispatcher<'static, 'static>,
}

impl State {
  pub fn new(world: World, dispatcher: Dispatcher<'static, 'static>) -> Self {
    Self {
      run_state: RunState::Running,
      world,
      dispatcher,
    }
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
    self.world.insert(DeltaTime(Duration::from_secs_f32(
      ctx.frame_time_ms / 1000.0,
    )));

    if let Some(key) = ctx.key {
      match key {
        // runtime controls
        VirtualKeyCode::Escape => ctx.quit(),
        VirtualKeyCode::Space => self.run_state = !self.run_state,

        _ => (),
      }
    }

    self.dispatcher.dispatch(&self.world);
    self.world.maintain();

    self.world.exec(
      |(positions, renderables): (ReadStorage<Position>, ReadStorage<Renderable>)| {
        ctx.cls();

        for (pos, render) in (&positions, &renderables).join() {
          ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
      },
    );
  }
}

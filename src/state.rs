use crate::RunState;
use bracket_lib::prelude::*;
use components::*;
use legion::prelude::*;
use resources::*;
use std::time::Duration;

pub struct State {
  pub run_state: RunState,
  pub world: World,
  pub resources: Resources,
  pub schedule: Schedule,
}

impl State {
  fn render(&mut self, ctx: &mut BTerm) {
    ctx.cls();

    let query = <(Read<Position>, Read<Renderable>)>::query();
    for (pos, render) in query.iter(&self.world) {
      ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    }
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
    self.resources.insert(DeltaTime(Duration::from_secs_f32(
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

    self.schedule.execute(&mut self.world, &mut self.resources);

    self.render(ctx);
  }
}

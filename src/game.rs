use bracket_lib::prelude::*;
use components::*;
use legion::prelude::*;
use resources::*;
use std::time::Duration;

pub trait VaultkeeperState: std::fmt::Debug {
  fn on_start(&mut self, _term: &mut BTerm, _context: &mut WorldContext) {}

  fn on_stop(&mut self, _term: &mut BTerm, _context: &mut WorldContext) {}

  fn on_pause(&mut self, _term: &mut BTerm, _context: &mut WorldContext) {}

  fn on_resume(&mut self, _term: &mut BTerm, _context: &mut WorldContext) {}

  fn update(&mut self, _term: &mut BTerm, _context: &mut WorldContext) -> Transition {
    Transition::None
  }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Transition {
  None,
  Push(Box<dyn VaultkeeperState>),
  Switch(Box<dyn VaultkeeperState>),
  Pop,
  Quit,
}

pub struct WorldContext {
  pub world: World,
  pub resources: Resources,
}

pub struct Game {
  pub schedule: Schedule,
  pub context: WorldContext,
  pub state_stack: Vec<Box<dyn VaultkeeperState>>,
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
      .insert(DeltaTime(Duration::from_secs_f32(
        term.frame_time_ms / 1000.0,
      )));

    term.cls();

    if let Some(VirtualKeyCode::Escape) = term.key {
      // TODO: signal states to shut down (save, etc)
      term.quit();
      return;
    }

    self
      .schedule
      .execute(&mut self.context.world, &mut self.context.resources);

    if let Some(mut active) = self.state_stack.pop() {
      let transition = active.update(term, &mut self.context);

      if !matches!(transition, Transition::None) {
        log::debug!("{:?}", transition);
      }

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

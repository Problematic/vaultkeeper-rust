use super::Transition;
use crate::ui::Input;
use legion::prelude::{Resources, World};

pub struct WorldContext {
  pub world: World,
  pub resources: Resources,
}

pub trait State<TContext> {
  fn on_start(&mut self, _context: &mut TContext) {}

  fn on_stop(&mut self, _context: &mut TContext) {}

  fn on_pause(&mut self, _context: &mut TContext) {}

  fn on_resume(&mut self, _context: &mut TContext) {}

  fn update(&mut self, _context: &mut TContext) -> Transition<TContext> {
    Transition::None
  }

  fn handle_input(&mut self, _context: &mut TContext, _input: Input) -> Transition<TContext> {
    Transition::None
  }
}

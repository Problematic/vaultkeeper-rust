use super::Transition;
use bracket_lib::prelude::BTerm;
use legion::prelude::{Resources, World};

pub struct WorldContext {
  pub world: World,
  pub resources: Resources,
}

pub trait State {
  fn on_start(&mut self, _term: &mut BTerm, _context: &mut WorldContext) {}

  fn on_stop(&mut self, _term: &mut BTerm, _context: &mut WorldContext) {}

  fn on_pause(&mut self, _term: &mut BTerm, _context: &mut WorldContext) {}

  fn on_resume(&mut self, _term: &mut BTerm, _context: &mut WorldContext) {}

  fn update(&mut self, _term: &mut BTerm, _context: &mut WorldContext) -> Transition {
    Transition::None
  }
}

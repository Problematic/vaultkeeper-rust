use bracket_lib::prelude::BTerm;

pub trait Render {
  // TODO: take camera
  fn render(&self, term: &mut BTerm);
}

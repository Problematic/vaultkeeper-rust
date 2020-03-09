use bracket_lib::prelude::*;
use specs::{prelude::*, Component};

#[derive(Component)]
pub struct Renderable {
  pub glyph: u8,
  pub fg: RGB,
  pub bg: RGB,
}

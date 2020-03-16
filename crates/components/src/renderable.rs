use bracket_lib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Renderable {
  pub glyph: u8,
  pub fg: RGB,
  pub bg: RGB,
}

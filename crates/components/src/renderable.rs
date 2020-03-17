use bracket_lib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Renderable {
  pub glyph: FontCharType,
  pub colors: ColorPair,
}

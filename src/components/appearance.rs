use bracket_lib::prelude::{to_cp437, ColorPair, FontCharType, RGBA};

#[derive(Debug, Clone, Copy)]
pub struct Appearance {
  pub glyph: FontCharType,
  pub colors: ColorPair,
}

impl Appearance {
  pub fn new<FG: Into<RGBA>, BG: Into<RGBA>>(glyph: char, fg: FG, bg: BG) -> Self {
    Self {
      glyph: to_cp437(glyph),
      colors: ColorPair::new(fg, bg),
    }
  }
}

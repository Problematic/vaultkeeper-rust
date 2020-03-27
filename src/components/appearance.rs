use bracket_lib::prelude::{to_cp437, FontCharType, RGBA};

#[derive(Debug, Clone, Copy)]
pub struct Appearance {
  pub glyph: FontCharType,
  pub fg: RGBA,
  pub bg: RGBA,
}

impl Appearance {
  pub fn new<FG: Into<RGBA>, BG: Into<RGBA>>(glyph: char, fg: FG, bg: BG) -> Self {
    Self {
      glyph: to_cp437(glyph),
      fg: fg.into(),
      bg: bg.into(),
    }
  }
}

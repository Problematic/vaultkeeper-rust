use specs::{prelude::*, Component};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Need {
  Hunger,
  Social,
}

#[derive(Component, Debug, Default)]
pub struct Needs(pub HashMap<Need, f32>);

impl std::convert::From<Vec<(Need, f32)>> for Needs {
  fn from(entries: Vec<(Need, f32)>) -> Self {
    Self(entries.iter().cloned().collect())
  }
}

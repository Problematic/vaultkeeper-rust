use specs::{prelude::*, Component};
use std::borrow::Cow;

#[derive(Component, Debug, Clone)]
pub struct Name(Cow<'static, str>);

impl Name {
  pub fn new<S>(name: S) -> Self
  where
    S: Into<Cow<'static, str>>,
  {
    Self(name.into())
  }
}

use std::fmt;
impl fmt::Display for Name {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

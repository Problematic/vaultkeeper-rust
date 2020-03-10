use specs::{prelude::*, Component};

#[derive(Component, Debug)]
pub struct Name(pub String);

impl std::convert::From<&str> for Name {
  fn from(name: &str) -> Self {
    Self(name.to_string())
  }
}

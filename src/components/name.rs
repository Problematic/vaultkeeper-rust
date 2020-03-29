use std::fmt;

#[derive(Debug, Clone)]
pub struct Name(pub String);

impl Name {
  pub fn new(name: &str) -> Self {
    Self(name.to_string())
  }
}

impl fmt::Display for Name {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

use legion::prelude::Entity;

#[derive(Debug)]
pub struct Viewshed {
  pub range: i32,
  pub visible_entities: Vec<Entity>,
}

impl Default for Viewshed {
  fn default() -> Self {
    Self {
      range: 5,
      visible_entities: Vec::new(),
    }
  }
}

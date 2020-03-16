use super::Need;

#[derive(Debug)]
pub struct PointOfInterest {
  pub need: Need,
  pub range: i32,
  pub is_global: bool,
}

use crate::utils::clamp;

#[derive(Debug, Clone)]
pub struct Health {
  pub max: u32,
  pub current: u32,
}

impl Health {
  pub fn new(max: u32) -> Self {
    Self { max, current: max }
  }
}

impl std::ops::SubAssign<u32> for Health {
  fn sub_assign(&mut self, amount: u32) {
    self.current = self.current.saturating_sub(amount);
  }
}

impl std::ops::AddAssign<u32> for Health {
  fn add_assign(&mut self, amount: u32) {
    self.current = clamp(self.current + amount, 0, self.max)
  }
}

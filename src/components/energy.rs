#[derive(Debug, Default)]
pub struct Energy {
  current: u8,
}

impl Energy {
  pub const ACTION_COST: u8 = 240;

  pub fn new(initial: u8) -> Self {
    Self { current: initial }
  }

  pub fn can_act(&self) -> bool {
    self.current >= Self::ACTION_COST
  }

  pub fn gain(&mut self, amount: u8) -> bool {
    // TODO: clamp to max energy
    self.current = self.current.saturating_add(amount);

    self.current >= Self::ACTION_COST
  }

  pub fn spend(&mut self) {
    assert!(self.current >= Self::ACTION_COST);
    self.current -= Self::ACTION_COST;
  }
}

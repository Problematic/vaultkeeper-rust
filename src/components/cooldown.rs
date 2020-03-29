use std::marker::PhantomData;
use std::time::Duration;

#[derive(Debug)]
pub struct Cooldown<T>(pub Duration, PhantomData<T>);

impl<T> Cooldown<T> {
  pub fn new(duration: Duration) -> Self {
    Self(duration, PhantomData)
  }
}

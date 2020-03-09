pub trait Input<'a> {
  type Context: 'a;

  #[must_use]
  fn score(&self, context: &Self::Context) -> f32;
}

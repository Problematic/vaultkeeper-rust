#[derive(Debug, Default)]
pub struct ActionSet {
  pub decisions: Vec<Decision>,
}

impl ActionSet {
  #[must_use]
  pub fn evaluate(&self, context: &mut AIContext) -> Option<(&Decision, f32)> {
    // TODO: return a lightweight handle to a globally-registered decision
    // so we don't have the clone the name and/or the decision itself
    self
      .decisions
      .iter()
      .map(|d| {
        log::debug!(
          "\u{250c} {} is evaluating '{}'...",
          // context.agent.name,
          "someone",
          d.name
        );

        let score = d.score(context);

        log::debug!("\u{2514} Score => {}", score);

        (d, score)
      })
      .filter(|r| r.1 > 0.0)
      .max_by_key(|r| NotNan::from(r.1))
  }
}

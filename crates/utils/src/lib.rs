mod clamp;

pub use clamp::clamp;
use std::time::Duration;

pub const ZERO_DURATION: Duration = Duration::from_secs(0);

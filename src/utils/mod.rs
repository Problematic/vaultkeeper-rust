mod clamp;
pub mod geom;

pub use clamp::clamp;
use std::time::Duration;

pub const ZERO_DURATION: Duration = Duration::from_secs(0);

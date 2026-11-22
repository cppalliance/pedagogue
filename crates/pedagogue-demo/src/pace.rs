//! How fast the demo moves.

use std::time::Duration;

/// The time between two steps of a script at the live pace.
const STEP_DELAY: Duration = Duration::from_millis(250);

/// How long the typing dots show before the stand-in's reply at the live
/// pace.
const REPLY_DELAY: Duration = Duration::from_millis(900);

/// How fast a script plays and the stand-in replies.
///
/// # Examples
///
/// ```
/// use pedagogue_demo::Pace;
///
/// assert_eq!(Pace::default(), Pace::Live);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Pace {
    /// Steps play a quarter second apart, so the page builds block by
    /// block, and the typing dots show for a moment before a reply.
    #[default]
    Live,
    /// Nothing waits. This is the pace for tests and screenshots.
    Instant,
}

impl Pace {
    /// Returns the time between two steps of a script.
    pub(crate) fn step_delay(self) -> Duration {
        match self {
            Self::Live => STEP_DELAY,
            Self::Instant => Duration::ZERO,
        }
    }

    /// Returns how long the typing dots show before a reply.
    pub(crate) fn reply_delay(self) -> Duration {
        match self {
            Self::Live => REPLY_DELAY,
            Self::Instant => Duration::ZERO,
        }
    }
}

/// Waits for a delay. A delay of zero returns at once and touches no timer.
pub(crate) async fn pause(delay: Duration) {
    if !delay.is_zero() {
        tokio::time::sleep(delay).await;
    }
}

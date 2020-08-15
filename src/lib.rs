use ring_vec::RingVec;
use std::time::{Duration, Instant};

/// How many times an event may occur in a given [`Duration`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RateLimit {
    pub count: usize,
    pub period: Duration,
}

impl RateLimit {
    /// Creates a new [`RateLimit`] with the provided values, but only
    /// if the count is not zero (otherwise, what's the point?)
    pub fn new(count: usize, period: Duration) -> Option<RateLimit> {
        if count > 0 {
            Some(RateLimit { count, period })
        } else {
            None
        }
    }
}

/// Enforces a [`RateLimit`] by maintaining a ring vector of
/// timestamps capped at the [`RateLimit::count`].
pub struct RateLimiter {
    pub limit: RateLimit,
    readings: RingVec<Instant>,
}

impl RateLimiter {
    /// Create without preallocating storage. Ideal if it may go unused.
    pub fn new(limit: RateLimit) -> RateLimiter {
        RateLimiter { limit, readings: RingVec::new(limit.count) }
    }

    /// Create with preallocated storage. Ideal if you're likely to
    /// use it a lot to avoid resizing during fill.
    pub fn new_preallocated(limit: RateLimit) -> RateLimiter {
        RateLimiter { limit, readings: RingVec::new_preallocated(limit.count) }
    }

    /// Checks whether we're able to perform the event at this
    /// time. On success, logs the current time to count towards
    /// future enforcement.
    pub fn check(&mut self) -> bool { self.check_at(Instant::now()) }

    /// Like [`RateLimiter::check`], but you can provide an arbitrary
    /// timestamp (useful for tests!).
    ///
    /// Warning: do not go backwards in time, things will mess up.
    pub fn check_at(&mut self, instant: Instant) -> bool {
        if self.readings.push(instant).is_ok() { return true; }
        let reclaimed = self.sweep(instant);
        if reclaimed {
            self.readings.push(instant).unwrap();
        }
        reclaimed
    }

    /// Removes all readings before the period of our [`RateLimit`],
    /// relative to the provided [`Instant`].
    pub fn sweep(&mut self, instant: Instant) -> bool {
        let bench = instant - self.limit.period;
        let mut reclaimed = false;
        while let Some(x) = self.readings.peek() {
            if *x < bench {
                reclaimed = true;
                self.readings.pop();
            } else { break; }
        }
        reclaimed
    }
}

use ring_vec::RingVec;
use std::time::{Duration, Instant};

/// A [`RateLimit`] is how many times an event may occur in a given period of time.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct RateLimit {
    pub count: usize,
    pub period: Duration,
}

impl RateLimit {
    /// Creates a new [`RateLimit`] with the provided values.
    /// Warning: Panics if count is zero.
    pub fn new(count: usize, period: Duration) -> Option<RateLimit> {
        if count > 0 {
            Some(RateLimit { count, period })
        } else {
            None
        }
    }
}

/// A [`RateLimiter`] enforces a [`RateLimit`] by maintaining a ring
/// vector of timestamps capped at the [`RateLimit::count`].
pub struct RateLimiter {
    pub limit: RateLimit,
    readings: RingVec<Instant>,
}

impl RateLimiter {
    /// Creates a new `RateLimiter` without preallocating
    /// storage. Ideal if you might never use it.
    pub fn new(limit: RateLimit) -> RateLimiter {
        RateLimiter { limit, readings: RingVec::new(limit.count) }
    }

    /// Creates a new `RateLimiter` with preallocated storage. Ideal
    /// if you're likely to use it a lot to avoid resizing during fill.
    pub fn new_preallocated(limit: RateLimit) -> RateLimiter {
        RateLimiter { limit, readings: RingVec::new_preallocated(limit.count) }
    }

    /// Logs the current time with the [`RateLimiter`] and checks if
    /// the event falls within the rate limit.
    pub fn check(&mut self) -> bool { self.check_at(Instant::now()) }

    /// Like `check`, but you can provide an arbitrary timestamp
    /// (useful for tests!).  Promise that you'll only march forwards
    /// in time and we promise to return the correct answers.
    pub fn check_at(&mut self, instant: Instant) -> bool {
        if self.readings.push(instant).is_ok() { return true; }
        let reclaimed = self.sweep(instant);
        if reclaimed {
            self.readings.push(instant).unwrap();
        }
        reclaimed
    }

    /// Removes all the readings from longer than the contained
    /// [`RateLimit`]'s period ago, relative to the provided instant.
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use ring_vec::RingVec;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct RateLimit {
    pub count: usize,
    pub period: Duration,
}

impl RateLimit {
    pub fn new(count: usize, period: Duration) -> Option<RateLimit> {
        if count > 0 {
            Some(RateLimit { count, period })
        } else {
            None
        }
    }
}

pub struct RateLimiter {
    pub limit: RateLimit,
    readings: RingVec<Instant>,
}

impl RateLimiter {
    // If you expect to use this lightly
    pub fn new(limit: RateLimit) -> RateLimiter {
        RateLimiter { limit, readings: RingVec::new(limit.count) }
    }

    // If you expect to use this heavily
    pub fn new_preallocated(limit: RateLimit) -> RateLimiter {
        RateLimiter { limit, readings: RingVec::new_preallocated(limit.count) }
    }

    /// Check if you are still permitted to perform the action
    pub fn check(&mut self) -> bool { self.check_at(Instant::now()) }

    // Promise that you'll only march forwards in time and we promise
    // to return the correct answers.
    pub fn check_at(&mut self, instant: Instant) -> bool {
        if self.readings.push(instant).is_ok() { return true; }
        let reclaimed = self.delete_outdated(instant);
        if reclaimed {
            self.readings.push(instant).unwrap();
        }
        reclaimed
    }

    /// Removes all the readings from longer than period ago.
    pub fn delete_outdated(&mut self, instant: Instant) -> bool {
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

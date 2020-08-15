#![feature(test)]
use std::time::{Duration, Instant};
use simple_rate_limit::*;

#[test]
fn zero() {
    assert_eq!(None, RateLimit::new(0, Duration::from_secs(1)));
}

#[test]
fn one_second() {
    let rl = RateLimit::new(1, Duration::from_secs(1)).unwrap();
    let mut rler = RateLimiter::new(rl);
    assert_eq!(rler.check(), true);
    assert_eq!(rler.check(), false);
}

#[test]
fn one_nanosecond() {
    let rl = RateLimit::new(1, Duration::from_nanos(1)).unwrap();
    let mut rler = RateLimiter::new(rl);
    let then = Instant::now();
    assert_eq!(rler.check_at(then), true);
    let now = then + Duration::from_nanos(2);
    assert_eq!(rler.check_at(now), true);
}

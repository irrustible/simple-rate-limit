# simple-rate-limit

[![License](https://img.shields.io/crates/l/simple-rate-limit.svg)](https://github.com/irrustible/simple-rate-limit/blob/main/LICENSE)
[![Package](https://img.shields.io/crates/v/simple-rate-limit.svg)](https://crates.io/crates/simple-rate-limit)
[![Documentation](https://docs.rs/simple-rate-limit/badge.svg)](https://docs.rs/simple-rate-limit)

```rust
#[test]
fn one_nanosecond() {
    let rl = RateLimit::new(1, Duration::from_nanos(1)).unwrap();
    let mut rler = RateLimiter::new(rl);
    let then = Instant::now();
    assert_eq!(rler.check_at(then), true);
    let now = then + Duration::from_nanos(2);
    assert_eq!(rler.check_at(now), true);
}
```

## Status

Beta? Only has basic tests but seems to work.

## Copyright and License

    Copyright (c) 2020 James Laver, simple-rate-limit contributors.
    
    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/.

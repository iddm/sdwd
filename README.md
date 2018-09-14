# sdwd
[![](https://meritbadge.herokuapp.com/sdwd)](https://crates.io/crates/sdwd) [![](https://travis-ci.org/vityafx/sdwd.svg?branch=master)](https://travis-ci.org/vityafx/sdwd) [![](https://docs.rs/sdwd/badge.svg)](https://docs.rs/sdwd)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

`sdwd` - SystemD WatchDog crate. Provides simple watchdog notification utilities for easy watchdog
management.

# Usage

```rust
extern crate sdwd;

fn main() {
    let wd_thread = sdwd::start_watchdog_thread(sdwd::expected_timeout().unwrap());

    loop {
        use std::thread;
        thread::sleep_ms(5000);
        println!("Printing this message once in five seconds");
    }

    wd_thread.join();
}
```

## License

This project is [licensed under the MIT license](https://github.com/vityafx/sdwd/blob/master/LICENSE).

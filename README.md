# sdwd
[![](https://meritbadge.herokuapp.com/sdwd)](https://crates.io/crates/sdwd) [![](https://travis-ci.org/vityafx/sdwd.svg?branch=master)](https://travis-ci.org/vityafx/sdwd) [![](https://docs.rs/sdwd/badge.svg)](https://docs.rs/sdwd)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

`sdwd` - SystemD WatchDog crate. Provides simple watchdog notification utilities for easy watchdog
management.

# Usage

*sdwd-test.service*:

```service
[Unit]
Description=sdwd test daemon

[Service]
ExecStart=sdwd-test
WatchdogSec=30s
Restart=on-failure
```

*src/main.rs*:

```rust
extern crate sdwd;

fn main() {
    let recommended_timeout = sdwd::recommended_timeout().unwrap();
    println!("Recommended timeout: {:?}", recommended_timeout);
    let _ = sdwd::start_watchdog_thread(recommended_timeout);

    loop {
        use std::thread;
        use std::time::Duration;
        thread::sleep(Duration::from_secs(5));
        println!("Printing this message once in five seconds");
    }
}
```

## Read about watchdogs
https://www.freedesktop.org/software/systemd/man/systemd.service.html

https://www.freedesktop.org/software/systemd/man/sd_watchdog_enabled.html

## License

This project is [licensed under the MIT license](https://github.com/vityafx/sdwd/blob/master/LICENSE).

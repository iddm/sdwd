//! sdwd - SystemD WatchDog crate. Provides simple watchdog notification utilities for easy watchdog
//! management.
//!
//! # Usage
//!
//! ```rust,no_run
//! extern crate sdwd;
//!
//! fn main() {
//!     let wd_thread = sdwd::start_watchdog_thread(sdwd::expected_timeout().unwrap());
//!
//!     loop {
//!         use std::thread;
//!         thread::sleep_ms(5000);
//!         println!("Printing this message once in five seconds");
//!     }
//!
//!     wd_thread.join();
//! }
//! ```

#[macro_use]
extern crate log;
extern crate systemd;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref WATCHDOG_NOTIFICATION_HASHMAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(systemd::daemon::STATE_WATCHDOG, "1");
        m
    };
}

/// Returns expected watchdog ping timeout with microseconds precision.
pub fn expected_timeout() -> Result<Duration, systemd::Error> {
    systemd::daemon::watchdog_enabled(false).map(Duration::from_micros)
}

/// Pings the watchdog. Should be called repeatedly within the `expected_timeout()` interval.
pub fn ping_watchdog() -> Result<bool, systemd::Error> {
    systemd::daemon::notify(false, WATCHDOG_NOTIFICATION_HASHMAP.to_owned())
}

/// Starts watchdog thread pinging with specified interval.
pub fn start_watchdog_thread(duration: Duration) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        thread::sleep(duration);

        if let Err(e) = ping_watchdog() {
            warn!("Could not ping systemd's watchdog: {:?}", e);
        }
    })
}

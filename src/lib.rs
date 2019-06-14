//! sdwd - SystemD WatchDog crate. Provides simple watchdog notification utilities for easy watchdog
//! management.
//!
//! # Usage
//!
//! ```rust,no_run
//! fn main() {
//!     let recommended_timeout = sdwd::recommended_timeout().unwrap();
//!     println!("Recommended timeout: {:?}", recommended_timeout);
//!     let _ = sdwd::start_watchdog_thread(recommended_timeout);
//!
//!     loop {
//!         use std::thread;
//!         use std::time::Duration;
//!         thread::sleep(Duration::from_secs(5));
//!         println!("Printing this message once in five seconds");
//!     }
//! }
//! ```

#[macro_use]
extern crate log;
extern crate systemd;

use std::thread;
use std::time::Duration;

/// Returns expected watchdog ping timeout with microseconds precision.
pub fn expected_timeout() -> Result<Duration, systemd::Error> {
    systemd::daemon::watchdog_enabled(false).map(Duration::from_micros)
}

/// According to the documentation of the systemd, it is recommended to ping watchdog every half of the
/// `expected_timeout()`'s.
pub fn recommended_timeout() -> Result<Duration, systemd::Error> {
    Ok(expected_timeout()? / 2)
}

/// Pings the watchdog. Should be called repeatedly within the `expected_timeout()` interval.
pub fn ping_watchdog() -> Result<bool, systemd::Error> {
    systemd::daemon::notify(false, [(systemd::daemon::STATE_WATCHDOG, "1")].iter())
}

/// Starts watchdog thread pinging with specified interval.
pub fn start_watchdog_thread(duration: Duration) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        thread::sleep(duration);

        if let Err(e) = ping_watchdog() {
            warn!("Could not ping systemd's watchdog: {:?}", e);
        } else {
            debug!("Pinged systemd's watchdog.");
        }
    })
}

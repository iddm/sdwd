use std::thread;
use std::time;

/// Spawns watchdog thread pinging with specified interval.
fn spawn_watchdog_thread(duration: time::Duration) -> thread::JoinHandle {
    thread::spawn(|| {
        
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

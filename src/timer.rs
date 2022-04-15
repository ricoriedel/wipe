use std::thread::sleep;
use std::time::{Duration, Instant};

#[cfg(test)]
use mockall::automock;

/// Allows for periodic execution of code.
#[cfg_attr(test, automock)]
pub trait Timer {
    /// Sleep until the next tick starts.
    fn sleep(&mut self);

    /// Get the delay between ticks.
    fn delay(&self) -> Duration;
}

/// A simple [Timer] based on the system clock.
pub struct SimpleTimer {
    delay: Duration,
    last: Instant
}

impl SimpleTimer {
    pub fn new(delay: Duration) -> Self {
        Self {
            last: Instant::now(),
            delay,
        }
    }
}

impl Timer for SimpleTimer {
    fn sleep(&mut self) {
        let now = Instant::now();

        if self.last + self.delay > now {
            sleep(self.delay - (now - self.last));
        }
        self.last = Instant::now();
    }

    fn delay(&self) -> Duration {
        self.delay
    }
}
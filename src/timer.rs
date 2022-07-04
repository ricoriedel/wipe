use std::thread::sleep;
use std::time::{Duration, Instant};

#[cfg(test)]
use mockall::automock;

/// Allows for periodic execution of code.
#[cfg_attr(test, automock)]
pub trait Timer {
    /// Set the start time of the timer.
    fn set(&mut self);

    /// Get the elapsed time since calling [Timer::set].
    fn elapsed(&self) -> Duration;

    /// Sleep until the next tick starts.
    fn sleep(&mut self);
}

/// A simple [Timer] based on the system clock.
pub struct SimpleTimer {
    delay: Duration,
    start: Instant,
    last: Instant
}

impl SimpleTimer {
    pub fn new(delay: Duration) -> Self {
        Self {
            delay,
            start: Instant::now(),
            last: Instant::now(),
        }
    }
}

impl Timer for SimpleTimer {
    fn set(&mut self) {
        self.start = Instant::now();
        self.last = self.start;
    }

    fn elapsed(&self) -> Duration {
        Instant::now() - self.start
    }

    fn sleep(&mut self) {
        let now = Instant::now();

        if self.last + self.delay > now {
            sleep(self.delay - (now - self.last));
        }
        self.last = Instant::now();
    }
}
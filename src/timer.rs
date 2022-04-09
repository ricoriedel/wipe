use std::thread::sleep;
use std::time::{Duration, Instant};
use mockall::automock;

#[automock]
pub trait Timer {
    fn sleep(&mut self);

    fn delay(&self) -> Duration;
}

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
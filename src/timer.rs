use crate::Error;
use crate::Renderer;
use std::thread;
use std::time::{Duration, Instant};

/// A stub for the system clock.
#[cfg_attr(test, mockall::automock)]
pub trait Clock {
    /// Returns the current time.
    fn now(&self) -> Instant;

    /// Sleep for the given duration.
    fn sleep(&self, duration: Duration);
}

/// The implementation of [Clock].
#[derive(derive_more::Constructor)]
pub struct ClockImpl;

impl Clock for ClockImpl {
    fn now(&self) -> Instant {
        Instant::now()
    }

    fn sleep(&self, duration: Duration) {
        thread::sleep(duration)
    }
}

/// A timer for rendering.
#[derive(derive_more::Constructor)]
pub struct Timer<T> {
    clock: T,
    duration: Duration,
    delay: Duration,
}

impl<T: Clock> Timer<T> {
    /// Runs the animation main loop.
    pub fn run(&self, mut renderer: impl Renderer) -> Result<(), Error> {
        let start = self.clock.now();
        let mut now = start;

        renderer.begin()?;

        while now.duration_since(start) < self.duration {
            let step = now.duration_since(start).as_secs_f32() / self.duration.as_secs_f32();

            renderer.render(step)?;
            now = self.delay(now);
        }
        renderer.end()
    }

    /// Sleeps until the next frame starts.
    /// Returns the current time.
    fn delay(&self, begin: Instant) -> Instant {
        let end = self.clock.now();

        if self.delay > end.duration_since(begin) {
            self.clock.sleep(self.delay - end.duration_since(begin));
        }
        self.clock.now()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::MockRenderer;
    use mockall::predicate::eq;
    use mockall::Sequence;

    #[test]
    fn run_steps_correct() {
        let mut clock = MockClock::new();
        let clock_seq = &mut Sequence::new();
        let begin = Instant::now();

        clock
            .expect_now()
            .once()
            .return_const(begin)
            .in_sequence(clock_seq);
        clock
            .expect_now()
            .times(2)
            .return_const(begin + Duration::from_secs(10))
            .in_sequence(clock_seq);
        clock
            .expect_now()
            .times(2)
            .return_const(begin + Duration::from_secs(20))
            .in_sequence(clock_seq);

        let timer = Timer::new(clock, Duration::from_secs(20), Duration::from_secs(10));

        let mut renderer = MockRenderer::new();
        let renderer_seq = &mut Sequence::new();

        renderer
            .expect_begin()
            .once()
            .returning(|| Ok(()))
            .in_sequence(renderer_seq);
        renderer
            .expect_render()
            .with(eq(0.0))
            .once()
            .returning(|_| Ok(()))
            .in_sequence(renderer_seq);
        renderer
            .expect_render()
            .with(eq(0.5))
            .once()
            .returning(|_| Ok(()))
            .in_sequence(renderer_seq);
        renderer
            .expect_end()
            .once()
            .returning(|| Ok(()))
            .in_sequence(renderer_seq);

        timer.run(renderer).unwrap();
    }

    #[test]
    fn run_sleep_duration_correct() {
        let mut clock = MockClock::new();
        let clock_seq = &mut Sequence::new();
        let begin = Instant::now();

        clock
            .expect_now()
            .once()
            .return_const(begin)
            .in_sequence(clock_seq);
        clock
            .expect_now()
            .once()
            .return_const(begin + Duration::from_secs(4))
            .in_sequence(clock_seq);
        clock
            .expect_sleep()
            .once()
            .with(eq(Duration::from_secs(6)))
            .return_const(())
            .in_sequence(clock_seq);
        clock
            .expect_now()
            .once()
            .return_const(begin + Duration::from_secs(10))
            .in_sequence(clock_seq);

        let timer = Timer::new(clock, Duration::from_secs(10), Duration::from_secs(10));

        let mut renderer = MockRenderer::new();
        renderer.expect_begin().returning(|| Ok(()));
        renderer.expect_render().returning(|_| Ok(()));
        renderer.expect_end().returning(|| Ok(()));

        timer.run(renderer).unwrap();
    }

    #[test]
    fn run_delay_exceeded_does_not_sleep() {
        let mut clock = MockClock::new();
        let clock_seq = &mut Sequence::new();
        let begin = Instant::now();

        clock
            .expect_now()
            .once()
            .return_const(begin)
            .in_sequence(clock_seq);
        clock
            .expect_now()
            .times(2)
            .return_const(begin + Duration::from_secs(12))
            .in_sequence(clock_seq);

        let timer = Timer::new(clock, Duration::from_secs(10), Duration::from_secs(10));

        let mut renderer = MockRenderer::new();
        renderer.expect_begin().returning(|| Ok(()));
        renderer.expect_render().returning(|_| Ok(()));
        renderer.expect_end().returning(|| Ok(()));

        timer.run(renderer).unwrap();
    }
}

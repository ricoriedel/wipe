use std::time::Duration;
use anyhow::Error;
use crate::Renderer;
use crate::timer::Timer;

/// Periodically calls [Renderer::render] and [Renderer::present].
pub struct Runner<TTimer, TRenderer> {
    duration: Duration,
    timer: TTimer,
    renderer: TRenderer,
}

impl<T1: Timer, T2: Renderer> Runner<T1, T2> {
    pub fn new(duration: Duration,
               timer: T1,
               renderer: T2) -> Self {

        Self { duration, timer, renderer }
    }

    pub fn run(mut self) -> Result<(), Error> {
        self.timer.set();

        while self.timer.elapsed() < self.duration {
            let step = self.timer.elapsed().as_secs_f32() / self.duration.as_secs_f32();

            self.renderer.render(step);
            self.renderer.present()?;
            self.timer.sleep();
        }
        self.renderer.render(1.0);
        self.renderer.present()?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use mockall::predicate::*;
    use mockall::Sequence;
    use crate::timer::MockTimer;
    use crate::render::MockRenderer;
    use super::*;

    #[test]
    fn run() {
        let mut timer = MockTimer::new();
        let mut renderer = MockRenderer::new();
        let seq = &mut Sequence::new();

        timer.expect_set().once().in_sequence(seq).return_const(());

        timer.expect_elapsed().times(2).in_sequence(seq).return_const(Duration::from_secs(0));
        renderer.expect_render().once().with(eq(0.0)).in_sequence(seq).return_const(());
        renderer.expect_present().once().in_sequence(seq).returning(|| Ok(()));
        timer.expect_sleep().once().in_sequence(seq).return_const(());

        timer.expect_elapsed().times(2).in_sequence(seq).return_const(Duration::from_secs(2));
        renderer.expect_render().once().with(eq(0.5)).in_sequence(seq).return_const(());
        renderer.expect_present().once().in_sequence(seq).returning(|| Ok(()));
        timer.expect_sleep().once().in_sequence(seq).return_const(());

        timer.expect_elapsed().times(1).in_sequence(seq).return_const(Duration::from_secs(4));
        renderer.expect_render().once().with(eq(1.0)).in_sequence(seq).return_const(());
        renderer.expect_present().once().in_sequence(seq).returning(|| Ok(()));

        let runner = Runner::new(Duration::from_secs(4), timer, renderer);

        runner.run().unwrap();
    }
}
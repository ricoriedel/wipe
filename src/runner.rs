use std::time::Duration;
use anyhow::Error;
use crate::Renderer;
use crate::timer::Timer;

pub struct Runner<TTimer, TRenderer> {
    timer: TTimer,
    ticks: u128,
    renderer: TRenderer,
}

impl<T1: Timer, T2: Renderer> Runner<T1, T2> {
    pub fn new(duration: Duration,
               timer: T1,
               renderer: T2) -> Self {
        let ticks = duration.as_nanos() / timer.delay().as_nanos();

        Self { timer, ticks, renderer }
    }

    pub fn run(mut self) -> Result<(), Error> {
        for i in 0..=self.ticks {
            let step = i as f32 / self.ticks as f32;

            self.renderer.render(step);
            self.renderer.present()?;
            self.timer.sleep();
        }
        self.renderer.finish()
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

        timer.expect_delay().return_const(Duration::from_secs(2));

        renderer.expect_render().once().with(eq(0.0)).in_sequence(seq).return_const(());
        renderer.expect_present().once().in_sequence(seq).returning(|| Ok(()));
        timer.expect_sleep().once().in_sequence(seq).return_const(());

        renderer.expect_render().once().with(eq(0.5)).in_sequence(seq).return_const(());
        renderer.expect_present().once().in_sequence(seq).returning(|| Ok(()));
        timer.expect_sleep().once().in_sequence(seq).return_const(());

        renderer.expect_render().once().with(eq(1.0)).in_sequence(seq).return_const(());
        renderer.expect_present().once().in_sequence(seq).returning(|| Ok(()));
        timer.expect_sleep().once().in_sequence(seq).return_const(());

        renderer.expect_finish().once().in_sequence(seq).returning(|| Ok(()));

        let runner = Runner::new(Duration::from_secs(4), timer, renderer);

        runner.run().unwrap();
    }
}
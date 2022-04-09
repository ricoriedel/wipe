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
        for i in 0..self.ticks {
            let step = i as f32 / self.ticks as f32;

            self.renderer.render(step);
            self.renderer.present()?;
            self.timer.sleep();
        }
        self.renderer.finish()
    }
}
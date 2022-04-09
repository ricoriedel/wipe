use std::time::Duration;
use anyhow::Error;
use crate::Renderer;
use crate::timer::Timer;

pub struct Runner {
    timer: Box<dyn Timer>,
    ticks: u128,
    renderer: Box<dyn Renderer>,
}

impl Runner {
    pub fn new(duration: Duration,
               timer: Box<dyn Timer>,
               renderer: Box<dyn Renderer>) -> Self {
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
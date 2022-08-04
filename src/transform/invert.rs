use crate::pattern::*;
use crate::Vector;

#[derive(derive_more::Constructor)]
pub struct InvertFactory {
    child: Box<dyn PatternFactory>,
}

#[derive(derive_more::Constructor)]
pub struct Invert {
    child: Box<dyn Pattern>,
}

impl PatternFactory for InvertFactory {
    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        let mut copy = config.clone();
        copy.step = 1.0 - config.step;

        Box::new(Invert::new(self.child.create(&copy)))
    }
}

impl Pattern for Invert {
    fn sample(&self, pos: Vector) -> f32 {
        1.0 - self.child.sample(pos)
    }
}

use crate::pattern::*;
use crate::Vector;

#[derive(derive_more::Constructor)]
pub struct ShiftFactory {
    child: Box<dyn PatternFactory>,
}

#[derive(derive_more::Constructor)]
pub struct Shift {
    child: Box<dyn Pattern>,
    shift: f32,
}

impl PatternFactory for ShiftFactory {
    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        Box::new(Shift::new(self.child.create(config), config.step))
    }
}

impl Pattern for Shift {
    fn sample(&self, pos: Vector) -> f32 {
        self.child.sample(pos) + 1.0 - 2.0 * self.shift
    }
}

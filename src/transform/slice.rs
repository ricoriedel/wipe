use crate::pattern::*;
use crate::Vector;

pub struct SliceFactory {
    child: Box<dyn PatternFactory>,
    width: f32,
    rest: f32,
}

#[derive(derive_more::Constructor)]
pub struct Slice {
    child: Box<dyn Pattern>,
    width: f32,
    rest: f32,
}

impl SliceFactory {
    pub fn new(child: Box<dyn PatternFactory>, slices: u8) -> Self {
        let width = 1.0 / slices as f32;
        let rest = 1.0 - width;

        Self { child, width, rest }
    }
}

impl PatternFactory for SliceFactory {
    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        Box::new(Slice::new(self.child.create(config), self.width, self.rest))
    }
}

impl Pattern for Slice {
    fn sample(&self, pos: Vector) -> f32 {
        (self.child.sample(pos) - self.rest) / self.width
    }
}

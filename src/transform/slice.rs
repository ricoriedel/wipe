use crate::pattern::*;
use crate::Vector;

#[derive(derive_more::Constructor)]
pub struct SliceFactory {
    child: Box<dyn PatternFactory>,
    scale: u8,
}

#[derive(derive_more::Constructor)]
pub struct Slice {
    child: Box<dyn Pattern>,
    scale: u8,
}

impl PatternFactory for SliceFactory {
    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        Box::new(Slice::new(self.child.create(config), self.scale))
    }
}

impl Pattern for Slice {
    fn sample(&self, pos: Vector) -> f32 {
        self.child.sample(pos) * self.scale as f32
    }
}

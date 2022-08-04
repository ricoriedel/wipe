use crate::pattern::*;
use crate::Vector;

#[derive(derive_more::Constructor)]
pub struct SwapFactory {
    child: Box<dyn PatternFactory>,
}

#[derive(derive_more::Constructor)]
pub struct Swap {
    child: Box<dyn Pattern>,
}

impl PatternFactory for SwapFactory {
    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        let mut copy = config.clone();
        copy.size = config.size.swap();

        Box::new(Swap::new(self.child.create(&copy)))
    }
}

impl Pattern for Swap {
    fn sample(&self, pos: Vector) -> f32 {
        self.child.sample(pos.swap())
    }
}

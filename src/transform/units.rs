use crate::pattern::*;
use crate::Vector;

#[derive(derive_more::Constructor)]
pub struct UnitsFactory {
    child: Box<dyn PatternFactory>,
    units: u8,
}

#[derive(derive_more::Constructor)]
pub struct Units {
    child: Box<dyn Pattern>,
    units: u8,
}

impl PatternFactory for UnitsFactory {
    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        Box::new(Units::new(self.child.create(config), self.units))
    }
}

impl Pattern for Units {
    fn sample(&self, pos: Vector) -> f32 {
        let sample = self.child.sample(pos);

        if 0.0 <= sample && sample < 1.0 {
            (sample * self.units as f32) % 1.0
        } else {
            sample
        }
    }
}

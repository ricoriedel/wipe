use crate::pattern::{Config, Pattern, PatternFactory};
use crate::vec::Vector;
use std::f32::consts::PI;

#[derive(Default)]
pub struct WheelFactory;
pub struct Wheel {
    center: Vector,
}

impl PatternFactory for WheelFactory {
    fn name(&self) -> String {
        stringify!(Wheel).to_lowercase()
    }

    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        Box::new(Wheel::new(config))
    }
}

impl Wheel {
    pub fn new(config: &Config) -> Self {
        let center = config.size.center();

        Self { center }
    }
}

impl Pattern for Wheel {
    fn sample(&self, pos: Vector) -> f32 {
        ((pos - self.center).angle() + PI) / PI / 2.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn name() {
        assert_eq!("wheel", WheelFactory::default().name());
    }

    #[test]
    fn sample() {
        let config = Config {
            size: Vector::new(10.0, 20.0),
            step: 0.0,
        };
        let pattern = Wheel::new(&config);

        assert_abs_diff_eq!(0.0, pattern.sample(Vector::new(0.0, 9.0)), epsilon = 0.1);
        assert_abs_diff_eq!(1.0, pattern.sample(Vector::new(0.0, 10.0)), epsilon = 0.1);
        assert_abs_diff_eq!(0.5, pattern.sample(Vector::new(10.0, 10.0)), epsilon = 0.1);
        assert_abs_diff_eq!(0.75, pattern.sample(Vector::new(5.0, 20.0)), epsilon = 0.1);
    }
}

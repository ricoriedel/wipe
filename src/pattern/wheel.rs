use crate::pattern::*;
use crate::Vector;
use std::f32::consts::PI;

#[derive(derive_more::Constructor)]
pub struct WheelFactory;
pub struct Wheel {
    center: Vector,
}

impl PatternFactory for WheelFactory {
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
    fn sample() {
        let config = Config {
            size: Vector::new(10.0, 20.0),
            ..Config::default()
        };
        let pattern = WheelFactory::new().create(&config);

        assert_abs_diff_eq!(0.0, pattern.sample(Vector::new(0.0, 9.0)), epsilon = 0.1);
        assert_abs_diff_eq!(1.0, pattern.sample(Vector::new(0.0, 10.0)), epsilon = 0.1);
        assert_abs_diff_eq!(0.5, pattern.sample(Vector::new(10.0, 10.0)), epsilon = 0.1);
        assert_abs_diff_eq!(0.75, pattern.sample(Vector::new(5.0, 20.0)), epsilon = 0.1);
    }
}

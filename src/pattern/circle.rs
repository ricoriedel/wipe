use crate::pattern::{Config, Pattern, PatternFactory};
use crate::vec::Vector;

#[derive(Default)]
pub struct CircleFactory;
pub struct Circle {
    center: Vector,
    radius: f32,
}

impl PatternFactory for CircleFactory {
    fn name(&self) -> String {
        stringify!(Circle).to_lowercase()
    }

    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        Box::new(Circle::new(config))
    }
}

impl Circle {
    pub fn new(config: &Config) -> Self {
        let center = config.size.center();
        let radius = center.len();

        Self { center, radius }
    }
}

impl Pattern for Circle {
    fn sample(&self, pos: Vector) -> f32 {
        (pos - self.center).len() / self.radius
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn name() {
        assert_eq!("circle", CircleFactory::default().name());
    }

    #[test]
    fn sample() {
        let config = Config {
            size: Vector::new(10.0, 20.0),
            step: 0.0,
        };
        let pattern = Circle::new(&config);

        assert_abs_diff_eq!(1.0, pattern.sample(Vector::new(0.0, 0.0)), epsilon = 0.1);
        assert_abs_diff_eq!(0.0, pattern.sample(Vector::new(5.0, 10.0)), epsilon = 0.1);
        assert_abs_diff_eq!(0.5, pattern.sample(Vector::new(7.5, 15.0)), epsilon = 0.1);
    }
}

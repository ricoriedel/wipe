use crate::pattern::{Config, Pattern, PatternFactory};
use crate::vec::Vector;

#[derive(Default)]
pub struct RhombusFactory;
pub struct Rhombus {
    center: Vector,
    distance: f32,
}

impl PatternFactory for RhombusFactory {
    fn name(&self) -> String {
        stringify!(Rhombus).to_lowercase()
    }

    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        Box::new(Rhombus::new(config))
    }
}

impl Rhombus {
    pub fn new(config: &Config) -> Rhombus {
        let center = config.size.center();
        let distance = center.sum();

        Self { center, distance }
    }
}

impl Pattern for Rhombus {
    fn sample(&self, pos: Vector) -> f32 {
        (pos - self.center).abs().sum() / self.distance
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn name() {
        assert_eq!("rhombus", RhombusFactory::default().name());
    }
    
    #[test]
    fn sample() {
        let config = Config {
            size: Vector::new(10.0, 5.0),
            step: 0.0,
        };
        let pattern = Rhombus::new(&config);

        assert_abs_diff_eq!(1.0, pattern.sample(Vector::new(0.0, 0.0)), epsilon = 0.1);
        assert_abs_diff_eq!(1.0, pattern.sample(Vector::new(10.0, 5.0)), epsilon = 0.1);
        assert_abs_diff_eq!(0.0, pattern.sample(Vector::new(5.0, 2.5)), epsilon = 0.1);
        assert_abs_diff_eq!(0.5, pattern.sample(Vector::new(7.0, 0.5)), epsilon = 0.1);
    }
}
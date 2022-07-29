use crate::pattern::{Config, Pattern, PatternFactory};
use crate::vec::Vector;

#[derive(Default)]
pub struct LineFactory;
pub struct Line {
    width: f32,
}

impl PatternFactory for LineFactory {
    fn name(&self) -> String {
        stringify!(Line).to_lowercase()
    }

    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        Box::new(Line::new(config))
    }
}

impl Line {
    pub fn new(config: &Config) -> Self {
        let width = config.size.x;

        Self { width }
    }
}

impl Pattern for Line {
    fn sample(&self, pos: Vector) -> f32 {
        pos.x / self.width
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn name() {
        assert_eq!("line", LineFactory::default().name());
    }

    #[test]
    fn sample() {
        let config = Config {
            size: Vector::new(20.0, 0.0),
            step: 0.0,
        };
        let pattern = Line::new(&config);

        assert_abs_diff_eq!(0.0, pattern.sample(Vector::new(0.0, 4.0)), epsilon = 0.1);
        assert_abs_diff_eq!(0.4, pattern.sample(Vector::new(8.0, 8.0)), epsilon = 0.1);
        assert_abs_diff_eq!(0.8, pattern.sample(Vector::new(16.0, 7.0)), epsilon = 0.1);
        assert_abs_diff_eq!(1.0, pattern.sample(Vector::new(20.0, 3.0)), epsilon = 0.1);
    }
}

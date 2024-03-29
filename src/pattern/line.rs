use crate::pattern::*;
use crate::Vector;

/// A factory for [Line].
#[derive(derive_more::Constructor)]
pub struct LineFactory;
/// A horizontal line [Pattern].
pub struct Line {
    width: f32,
}

impl PatternFactory for LineFactory {
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
    fn sample() {
        let config = Config {
            size: Vector::new(20.0, 0.0),
            ..Config::default()
        };
        let pattern = LineFactory::new().create(&config);

        assert_abs_diff_eq!(0.0, pattern.sample(Vector::new(0.0, 4.0)), epsilon = 0.1);
        assert_abs_diff_eq!(0.4, pattern.sample(Vector::new(8.0, 8.0)), epsilon = 0.1);
        assert_abs_diff_eq!(0.8, pattern.sample(Vector::new(16.0, 7.0)), epsilon = 0.1);
        assert_abs_diff_eq!(1.0, pattern.sample(Vector::new(20.0, 3.0)), epsilon = 0.1);
    }
}

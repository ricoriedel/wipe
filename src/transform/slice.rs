use crate::pattern::*;
use crate::Vector;

#[derive(derive_more::Constructor)]
pub struct SliceFactory {
    child: Box<dyn PatternFactory>,
    slices: u8,
}

pub struct Slice {
    child: Box<dyn Pattern>,
    width: f32,
    rest: f32,
}

impl PatternFactory for SliceFactory {
    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        Box::new(Slice::new(self.child.create(config), self.slices))
    }
}

impl Slice {
    pub fn new(child: Box<dyn Pattern>, slices: u8) -> Self {
        let width = 1.0 / slices as f32;
        let rest = 1.0 - width;

        Self { child, width, rest }
    }
}

impl Pattern for Slice {
    fn sample(&self, pos: Vector) -> f32 {
        (self.child.sample(pos) - self.rest) / self.width
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::MockPatternFactory;
    use approx::*;
    use mockall::predicate::eq;

    #[test]
    fn create_config_correct() {
        let config = Config {
            size: Vector::new(7.0, 3.0),
            step: 0.2,
        };
        let mut child = MockPatternFactory::new();
        child
            .expect_create()
            .with(eq(config))
            .once()
            .returning(|_| Box::new(MockPattern::new()));

        SliceFactory::new(Box::new(child), 4).create(&config);
    }

    #[test]
    fn sample_starts_with_one() {
        let mut child = MockPatternFactory::new();
        child.expect_create().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler.expect_sample().return_const(1.0);
            Box::new(sampler)
        });

        let sampler = SliceFactory::new(Box::new(child), 4).create(&Config::default());

        assert_abs_diff_eq!(1.0, sampler.sample(Vector::default()));
    }

    #[test]
    fn sample_ends_with_zero() {
        let mut child = MockPatternFactory::new();
        child.expect_create().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler.expect_sample().return_const(0.75);
            Box::new(sampler)
        });

        let sampler = SliceFactory::new(Box::new(child), 4).create(&Config::default());

        assert_abs_diff_eq!(0.0, sampler.sample(Vector::default()));
    }

    #[test]
    fn sample_values_beyond_end_are_negative() {
        let mut child = MockPatternFactory::new();
        child.expect_create().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler.expect_sample().return_const(0.5);
            Box::new(sampler)
        });

        let sampler = SliceFactory::new(Box::new(child), 4).create(&Config::default());

        assert!(sampler.sample(Vector::default()) < 0.0);
    }

    #[test]
    fn sample_pos_correct() {
        let mut child = MockPatternFactory::new();
        child.expect_create().once().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler
                .expect_sample()
                .with(eq(Vector::new(3.0, 5.0)))
                .once()
                .return_const(0.0);
            Box::new(sampler)
        });

        let sampler = SliceFactory::new(Box::new(child), 3).create(&Config::default());

        sampler.sample(Vector::new(3.0, 5.0));
    }
}

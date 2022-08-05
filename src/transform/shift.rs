use crate::pattern::*;
use crate::Vector;

#[derive(derive_more::Constructor)]
pub struct ShiftFactory {
    child: Box<dyn PatternFactory>,
}

#[derive(derive_more::Constructor)]
pub struct Shift {
    child: Box<dyn Pattern>,
    shift: f32,
}

impl PatternFactory for ShiftFactory {
    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        Box::new(Shift::new(self.child.create(config), config.step))
    }
}

impl Pattern for Shift {
    fn sample(&self, pos: Vector) -> f32 {
        self.child.sample(pos) + 1.0 - 2.0 * self.shift
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::MockPatternFactory;
    use mockall::predicate::eq;

    #[test]
    fn create_config_correct() {
        let config = Config {
            size: Vector::new(4.0, 2.0),
            step: 0.4,
        };
        let mut child = MockPatternFactory::new();
        child
            .expect_create()
            .with(eq(config))
            .once()
            .returning(|_| Box::new(MockPattern::new()));

        ShiftFactory::new(Box::new(child)).create(&config);
    }

    #[test]
    fn sample_shifted() {
        let config = Config {
            size: Vector::default(),
            step: 0.4,
        };
        let mut child = MockPatternFactory::new();
        child.expect_create().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler.expect_sample().return_const(0.6);
            Box::new(sampler)
        });

        let sampler = ShiftFactory::new(Box::new(child)).create(&config);

        assert_eq!(0.8, sampler.sample(Vector::default()));
    }

    #[test]
    fn sample_pos_correct() {
        let mut child = MockPatternFactory::new();
        child.expect_create().once().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler
                .expect_sample()
                .with(eq(Vector::new(6.0, 7.0)))
                .once()
                .return_const(0.0);
            Box::new(sampler)
        });

        let sampler = ShiftFactory::new(Box::new(child)).create(&Config::default());

        sampler.sample(Vector::new(6.0, 7.0));
    }
}

use crate::pattern::*;
use crate::Vector;

#[derive(derive_more::Constructor)]
pub struct InvertFactory {
    child: Box<dyn PatternFactory>,
}

#[derive(derive_more::Constructor)]
pub struct Invert {
    child: Box<dyn Pattern>,
}

impl PatternFactory for InvertFactory {
    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        let mut copy = config.clone();
        copy.step = 1.0 - config.step;

        Box::new(Invert::new(self.child.create(&copy)))
    }
}

impl Pattern for Invert {
    fn sample(&self, pos: Vector) -> f32 {
        1.0 - self.child.sample(pos)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::MockPatternFactory;
    use mockall::predicate::eq;

    #[test]
    fn create_config_correct() {
        let input = Config {
            size: Vector::new(4.0, 2.0),
            step: 0.4,
        };
        let mut output = input.clone();
        output.step = 0.6;

        let mut child = MockPatternFactory::new();
        child
            .expect_create()
            .with(eq(output))
            .once()
            .returning(|_| Box::new(MockPattern::new()));

        InvertFactory::new(Box::new(child)).create(&input);
    }

    #[test]
    fn sample_inverted() {
        let mut child = MockPatternFactory::new();
        child.expect_create().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler.expect_sample().return_const(0.7);
            Box::new(sampler)
        });

        let sampler = InvertFactory::new(Box::new(child)).create(&Config::default());

        assert_eq!(0.3, sampler.sample(Vector::default()));
    }

    #[test]
    fn sample_pos_correct() {
        let mut child = MockPatternFactory::new();
        child.expect_create().once().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler
                .expect_sample()
                .with(eq(Vector::new(4.0, 2.0)))
                .once()
                .return_const(0.0);
            Box::new(sampler)
        });

        let sampler = InvertFactory::new(Box::new(child)).create(&Config::default());

        sampler.sample(Vector::new(4.0, 2.0));
    }
}
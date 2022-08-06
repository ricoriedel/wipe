use crate::pattern::*;
use crate::Vector;

/// A factory for [Swap].
///
/// Swaps the x-axis and y-axis of terminal size for the contained [Pattern].
#[derive(derive_more::Constructor)]
pub struct SwapFactory {
    child: Box<dyn PatternFactory>,
}

/// Swaps the x-axis and y-axis.
#[derive(derive_more::Constructor)]
pub struct Swap {
    child: Box<dyn Pattern>,
}

impl PatternFactory for SwapFactory {
    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        let mut copy = config.clone();
        copy.size = config.size.swap();

        Box::new(Swap::new(self.child.create(&copy)))
    }
}

impl Pattern for Swap {
    fn sample(&self, pos: Vector) -> f32 {
        self.child.sample(pos.swap())
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
        let input = Config {
            size: Vector::new(4.0, 2.0),
            step: 0.4,
        };
        let mut output = input.clone();
        output.size = Vector::new(2.0, 4.0);

        let mut child = MockPatternFactory::new();
        child
            .expect_create()
            .with(eq(output))
            .once()
            .returning(|_| Box::new(MockPattern::new()));

        SwapFactory::new(Box::new(child)).create(&input);
    }

    #[test]
    fn sample_value_correct() {
        let mut child = MockPatternFactory::new();
        child.expect_create().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler.expect_sample().return_const(0.4);
            Box::new(sampler)
        });

        let sampler = SwapFactory::new(Box::new(child)).create(&Config::default());

        assert_abs_diff_eq!(0.4, sampler.sample(Vector::default()));
    }

    #[test]
    fn sample_pos_correct() {
        let mut child = MockPatternFactory::new();
        child.expect_create().once().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler
                .expect_sample()
                .with(eq(Vector::new(9.0, 5.0)))
                .once()
                .return_const(0.0);
            Box::new(sampler)
        });

        let sampler = SwapFactory::new(Box::new(child)).create(&Config::default());

        sampler.sample(Vector::new(5.0, 9.0));
    }
}

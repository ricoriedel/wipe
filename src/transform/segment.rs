use crate::pattern::*;
use crate::Vector;

/// A factory for [Segments].
#[derive(derive_more::Constructor)]
pub struct SegmentsFactory {
    child: Box<dyn PatternFactory>,
    segments: f32,
}

/// Converts a pattern to `n` segments each starting with zero and ending with one.
#[derive(derive_more::Constructor)]
pub struct Segments {
    child: Box<dyn Pattern>,
    segments: f32,
}

impl PatternFactory for SegmentsFactory {
    fn create(&self, config: &Config) -> Box<dyn Pattern> {
        Box::new(Segments::new(self.child.create(config), self.segments))
    }
}

impl Pattern for Segments {
    fn sample(&self, pos: Vector) -> f32 {
        let sample = self.child.sample(pos);

        if 0.0 <= sample && sample < 1.0 {
            sample * self.segments % 1.0
        } else {
            sample
        }
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
            size: Vector::new(6.0, 3.0),
            step: 0.4,
        };
        let mut child = MockPatternFactory::new();
        child
            .expect_create()
            .with(eq(config))
            .once()
            .returning(|_| Box::new(MockPattern::new()));

        SegmentsFactory::new(Box::new(child), 2.0).create(&config);
    }

    #[test]
    fn sample_above_one_untouched() {
        let mut child = MockPatternFactory::new();
        child.expect_create().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler.expect_sample().return_const(1.1);
            Box::new(sampler)
        });

        let sampler = SegmentsFactory::new(Box::new(child), 3.0).create(&Config::default());

        assert_abs_diff_eq!(1.1, sampler.sample(Vector::default()));
    }

    #[test]
    fn sample_below_zero_untouched() {
        let mut child = MockPatternFactory::new();
        child.expect_create().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler.expect_sample().return_const(-0.1);
            Box::new(sampler)
        });

        let sampler = SegmentsFactory::new(Box::new(child), 3.0).create(&Config::default());

        assert_abs_diff_eq!(-0.1, sampler.sample(Vector::default()));
    }

    #[test]
    fn sample_second_segment_begins_with_one() {
        let mut child = MockPatternFactory::new();
        child.expect_create().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler.expect_sample().return_const(0.74);
            Box::new(sampler)
        });

        let sampler = SegmentsFactory::new(Box::new(child), 4.0).create(&Config::default());

        assert_abs_diff_eq!(0.96, sampler.sample(Vector::default()), epsilon = 0.01);
    }

    #[test]
    fn sample_second_segment_ends_with_zero() {
        let mut child = MockPatternFactory::new();
        child.expect_create().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler.expect_sample().return_const(0.5);
            Box::new(sampler)
        });

        let sampler = SegmentsFactory::new(Box::new(child), 4.0).create(&Config::default());

        assert_abs_diff_eq!(0.0, sampler.sample(Vector::default()));
    }

    #[test]
    fn sample_last_segment_begins_with_one() {
        let mut child = MockPatternFactory::new();
        child.expect_create().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler.expect_sample().return_const(0.24);
            Box::new(sampler)
        });

        let sampler = SegmentsFactory::new(Box::new(child), 4.0).create(&Config::default());

        assert_abs_diff_eq!(0.96, sampler.sample(Vector::default()));
    }

    #[test]
    fn sample_last_segment_ends_with_zero() {
        let mut child = MockPatternFactory::new();
        child.expect_create().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler.expect_sample().return_const(0.0);
            Box::new(sampler)
        });

        let sampler = SegmentsFactory::new(Box::new(child), 4.0).create(&Config::default());

        assert_abs_diff_eq!(0.0, sampler.sample(Vector::default()));
    }

    #[test]
    fn sample_pos_correct() {
        let mut child = MockPatternFactory::new();
        child.expect_create().once().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler
                .expect_sample()
                .with(eq(Vector::new(5.0, 1.0)))
                .once()
                .return_const(0.0);
            Box::new(sampler)
        });

        let sampler = SegmentsFactory::new(Box::new(child), 3.0).create(&Config::default());

        sampler.sample(Vector::new(5.0, 1.0));
    }
}

mod circle;
mod line;
mod rhombus;
mod wheel;

pub use circle::*;
pub use line::*;
pub use rhombus::*;
pub use wheel::*;

use crate::Vector;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Config {
    pub size: Vector,
    pub step: f32,
}

#[cfg_attr(test, mockall::automock)]
pub trait PatternFactory {
    fn create(&self, config: &Config) -> Box<dyn Pattern>;
}

#[cfg_attr(test, mockall::automock)]
pub trait Pattern {
    fn sample(&self, pos: Vector) -> f32;
}

#[cfg_attr(test, mockall::automock(type Sampler = MockSampler;))]
pub trait SamplerFactory {
    type Sampler: Sampler;

    fn create(&self, config: &Config) -> Self::Sampler;
}

#[cfg_attr(test, mockall::automock)]
pub trait Sampler {
    fn char(&self, pos: Vector) -> f32;

    fn color(&self, pos: Vector) -> f32;
}

#[derive(derive_more::Constructor)]
pub struct SamplerFactoryImpl {
    char: Box<dyn PatternFactory>,
    color: Box<dyn PatternFactory>,
}

#[derive(derive_more::Constructor)]
pub struct SamplerImpl {
    char: Box<dyn Pattern>,
    color: Box<dyn Pattern>,
}

impl SamplerFactory for SamplerFactoryImpl {
    type Sampler = SamplerImpl;

    fn create(&self, config: &Config) -> Self::Sampler {
        SamplerImpl::new(self.char.create(config), self.color.create(config))
    }
}

impl Sampler for SamplerImpl {
    fn char(&self, pos: Vector) -> f32 {
        self.char.sample(pos)
    }

    fn color(&self, pos: Vector) -> f32 {
        self.color.sample(pos)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;
    use mockall::predicate::eq;

    #[test]
    fn char() {
        let mut char = MockPattern::new();
        let color = MockPattern::new();

        char.expect_sample()
            .with(eq(Vector::new(2.0, 5.0)))
            .return_const(2.5);

        let sampler = SamplerImpl::new(Box::new(char), Box::new(color));

        assert_abs_diff_eq!(2.5, sampler.char(Vector::new(2.0, 5.0)));
    }

    #[test]
    fn color() {
        let char = MockPattern::new();
        let mut color = MockPattern::new();

        color
            .expect_sample()
            .with(eq(Vector::new(4.0, 2.0)))
            .return_const(3.2);

        let sampler = SamplerImpl::new(Box::new(char), Box::new(color));

        assert_abs_diff_eq!(3.2, sampler.color(Vector::new(4.0, 2.0)));
    }

    #[test]
    fn factory() {
        let mut char = MockPatternFactory::new();
        let mut color = MockPatternFactory::new();
        let config = Config {
            size: Vector::new(2.0, 3.0),
            step: 0.6,
        };

        char.expect_create().with(eq(config)).once().returning(|_| {
            let mut sampler = MockPattern::new();
            sampler.expect_sample().return_const(3.0);
            Box::new(sampler)
        });
        color
            .expect_create()
            .with(eq(config))
            .once()
            .returning(|_| {
                let mut sampler = MockPattern::new();
                sampler.expect_sample().return_const(5.0);
                Box::new(sampler)
            });

        let factory = SamplerFactoryImpl::new(Box::new(char), Box::new(color));
        let sampler = factory.create(&config);

        assert_abs_diff_eq!(3.0, sampler.char(Vector::default()));
        assert_abs_diff_eq!(5.0, sampler.color(Vector::default()));
    }
}

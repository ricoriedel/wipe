//! Contains all pattern traits and base patterns.

mod circle;
mod line;
mod rhombus;
mod wheel;

pub use circle::*;
pub use line::*;
pub use rhombus::*;
pub use wheel::*;

use crate::Vector;

/// A configuration for a [Pattern].
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Config {
    /// The size of the terminal.
    pub size: Vector,
    /// The current state of the animation.
    pub step: f32,
}

/// A factory to create a [Pattern].
#[cfg_attr(test, mockall::automock)]
pub trait PatternFactory {
    /// Creates a new [Pattern] with the given configuration.
    fn create(&self, config: &Config) -> Box<dyn Pattern>;
}

/// A pattern for an animation.
#[cfg_attr(test, mockall::automock)]
pub trait Pattern {
    /// Returns the level for a given coordinate.
    /// If it is a base pattern, the start position of the
    /// animation should by zero and the end position should be one.
    fn sample(&self, pos: Vector) -> f32;
}

/// A factor for a [Sampler].
#[cfg_attr(test, mockall::automock(type Sampler = MockSampler;))]
pub trait SamplerFactory {
    /// The type of the [Sampler].
    type Sampler: Sampler;

    /// Creates a new [Sampler].
    fn create(&self, config: &Config) -> Self::Sampler;
}

/// A sampler for multiple values.
#[cfg_attr(test, mockall::automock)]
pub trait Sampler {
    /// Returns the char level for a given position.
    fn char(&self, pos: Vector) -> f32;

    /// Returns the color level for a given position.
    fn color(&self, pos: Vector) -> f32;
}

/// The implementation of [SamplerFactory].
#[derive(derive_more::Constructor)]
pub struct SamplerFactoryImpl {
    char: Box<dyn PatternFactory>,
    color: Box<dyn PatternFactory>,
}

/// The implementation of [Sampler].
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

use crate::convert::char::CharSample;
use crate::convert::Converter;
use crate::pattern::{Config, Pattern, PatternFactory};
use crate::vec::Vector;
use crossterm::style::Color;
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Sample {
    Keep,
    Draw { char: char, color: Color },
    Clear,
}

pub trait SamplerFactory {
    fn create(&self, config: &Config) -> Box<dyn Sampler>;
}

pub trait Sampler {
    fn sample(&self, pos: Vector) -> Sample;
}

#[derive(derive_more::Constructor)]
pub struct SamplerFactoryImpl<T> {
    char: Box<dyn PatternFactory>,
    color: Box<dyn PatternFactory>,
    converter: Rc<T>,
}

#[derive(derive_more::Constructor)]
pub struct SamplerImpl<T> {
    char: Box<dyn Pattern>,
    color: Box<dyn Pattern>,
    converter: Rc<T>,
}

impl<T: Converter + 'static> SamplerFactory for SamplerFactoryImpl<T> {
    fn create(&self, config: &Config) -> Box<dyn Sampler> {
        Box::new(SamplerImpl::new(
            self.char.create(config),
            self.color.create(config),
            self.converter.clone(),
        ))
    }
}

impl<T: Converter> Sampler for SamplerImpl<T> {
    fn sample(&self, pos: Vector) -> Sample {
        match self.converter.char(self.char.sample(pos)) {
            CharSample::Keep => Sample::Keep,
            CharSample::Draw(char) => Sample::Draw {
                char,
                color: self.converter.color(self.color.sample(pos)),
            },
            CharSample::Clear => Sample::Clear,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::convert::MockConverter;
    use crate::pattern::MockPattern;
    use mockall::predicate::eq;

    #[test]
    fn sample_keep() {
        let color = MockPattern::new();
        let mut char = MockPattern::new();
        let mut converter = MockConverter::new();

        char.expect_sample().return_const(3.0);
        converter.expect_char().return_const(CharSample::Keep);

        let sampler = SamplerImpl::new(Box::new(char), Box::new(color), Rc::new(converter));

        assert_eq!(Sample::Keep, sampler.sample(Vector::default()));
    }

    #[test]
    fn sample_draw() {
        let mut char = MockPattern::new();
        let mut color = MockPattern::new();
        let mut converter = MockConverter::new();

        char.expect_sample().return_const(3.0);
        color.expect_sample().return_const(2.0);
        converter.expect_char().return_const(CharSample::Draw('M'));
        converter.expect_color().return_const(Color::Red);

        let sampler = SamplerImpl::new(Box::new(char), Box::new(color), Rc::new(converter));

        assert_eq!(
            Sample::Draw {
                char: 'M',
                color: Color::Red
            },
            sampler.sample(Vector::default())
        );
    }

    #[test]
    fn sample_clear() {
        let color = MockPattern::new();
        let mut char = MockPattern::new();
        let mut converter = MockConverter::new();

        char.expect_sample().return_const(3.0);
        converter.expect_char().return_const(CharSample::Clear);

        let sampler = SamplerImpl::new(Box::new(char), Box::new(color), Rc::new(converter));

        assert_eq!(Sample::Clear, sampler.sample(Vector::default()));
    }

    #[test]
    fn sample_args_correct() {
        let mut char = MockPattern::new();
        let mut color = MockPattern::new();
        let mut converter = MockConverter::new();

        char.expect_sample()
            .with(eq(Vector::new(4.0, 2.0)))
            .return_const(6.0);
        color
            .expect_sample()
            .with(eq(Vector::new(4.0, 2.0)))
            .return_const(7.0);
        converter
            .expect_char()
            .with(eq(6.0))
            .return_const(CharSample::Draw('A'));
        converter
            .expect_color()
            .with(eq(7.0))
            .return_const(Color::Reset);

        SamplerImpl::new(Box::new(char), Box::new(color), Rc::new(converter))
            .sample(Vector::new(4.0, 2.0));
    }
}

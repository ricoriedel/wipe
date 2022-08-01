mod char;
mod color;

pub use crate::convert::char::*;
pub use crate::convert::color::*;

use crossterm::style::Color;

#[cfg_attr(test, mockall::automock)]
pub trait Converter {
    fn char(&self, level: f32) -> CharSample;
    fn color(&self, level: f32) -> Color;
}

#[derive(derive_more::Constructor)]
pub struct ConverterImpl<T1, T2> {
    char: T1,
    color: T2,
}

impl<T1: CharConverter, T2: ColorConverter> Converter for ConverterImpl<T1, T2> {
    fn char(&self, level: f32) -> CharSample {
        self.char.convert(level)
    }

    fn color(&self, level: f32) -> Color {
        self.color.convert(level)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::convert::MockCharConverter;
    use crate::convert::MockColorConverter;
    use mockall::predicate::*;

    #[test]
    fn char() {
        let mut char = MockCharConverter::new();
        let color = MockColorConverter::new();

        char.expect_convert()
            .with(eq(4.0))
            .return_const(CharSample::Draw('M'));

        let converter = ConverterImpl::new(char, color);

        assert_eq!(CharSample::Draw('M'), converter.char(4.0));
    }

    #[test]
    fn color() {
        let char = MockCharConverter::new();
        let mut color = MockColorConverter::new();

        color
            .expect_convert()
            .with(eq(2.0))
            .return_const(Color::Yellow);

        let converter = ConverterImpl::new(char, color);

        assert_eq!(Color::Yellow, converter.color(2.0));
    }
}

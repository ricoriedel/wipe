use crate::convert::char::CharConverter;
use crate::convert::color::ColorConverter;
use crate::convert::level::{Level, LevelConverter};
use crossterm::style::Color;

mod char;
mod color;
mod level;

pub trait Converter {
    fn char(&self, level: f32) -> char;
    fn color(&self, level: f32) -> Color;
    fn level(&self, level: f32) -> Level;
}

pub struct ConverterImpl<T1, T2, T3> {
    char: T1,
    color: T2,
    level: T3,
}

impl<T1, T2, T3> ConverterImpl<T1, T2, T3> {
    pub fn new(char: T1, color: T2, level: T3) -> Self {
        Self { char, color, level }
    }
}

impl<T1: CharConverter, T2: ColorConverter, T3: LevelConverter> Converter
    for ConverterImpl<T1, T2, T3>
{
    fn char(&self, level: f32) -> char {
        self.char.convert(level)
    }

    fn color(&self, level: f32) -> Color {
        self.color.convert(level)
    }

    fn level(&self, level: f32) -> Level {
        self.level.convert(level)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::convert::char::MockCharConverter;
    use crate::convert::color::MockColorConverter;
    use crate::convert::level::MockLevelConverter;
    use mockall::predicate::eq;

    #[test]
    fn char() {
        let mut char = MockCharConverter::new();
        let color = MockColorConverter::new();
        let level = MockLevelConverter::new();

        char.expect_convert().with(eq(4.0)).return_const('M');

        let converter = ConverterImpl::new(char, color, level);

        assert_eq!('M', converter.char(4.0));
    }

    #[test]
    fn color() {
        let char = MockCharConverter::new();
        let mut color = MockColorConverter::new();
        let level = MockLevelConverter::new();

        color
            .expect_convert()
            .with(eq(2.0))
            .return_const(Color::Yellow);

        let converter = ConverterImpl::new(char, color, level);

        assert_eq!(Color::Yellow, converter.color(2.0));
    }

    #[test]
    fn level() {
        let char = MockCharConverter::new();
        let color = MockColorConverter::new();
        let mut level = MockLevelConverter::new();

        level
            .expect_convert()
            .with(eq(3.0))
            .return_const(Level::Draw(2.0));

        let converter = ConverterImpl::new(char, color, level);

        assert_eq!(Level::Draw(2.0), converter.level(3.0));
    }
}

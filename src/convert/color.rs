use crossterm::style::Color;

/// A trait to convert a sample to a [Color].
#[cfg_attr(test, mockall::automock)]
pub trait ColorConverter {
    fn convert(&self, level: f32) -> Color;
}

/// The implementation of [ColorConverter].
pub struct ColorConverterImpl {
    colors: Vec<Color>,
}

impl ColorConverterImpl {
    /// The colors used for mapping.
    pub fn new(colors: Vec<Color>) -> Self {
        Self { colors }
    }
}

impl ColorConverter for ColorConverterImpl {
    fn convert(&self, level: f32) -> Color {
        let len = self.colors.len() as f32;
        let index = (level * len).rem_euclid(len) as usize;

        self.colors[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crossterm::style::Color::*;

    #[test]
    fn convert_negative_index() {
        let converter = ColorConverterImpl::new(vec![Red, Green, Blue]);

        assert_eq!(Blue, converter.convert(-0.2));
    }

    #[test]
    fn convert_index_zero() {
        let converter = ColorConverterImpl::new(vec![Red, Green, Blue]);

        assert_eq!(Red, converter.convert(0.0));
    }

    #[test]
    fn convert() {
        let converter = ColorConverterImpl::new(vec![Red, Green, Blue]);

        assert_eq!(Green, converter.convert(0.5));
    }

    #[test]
    fn convert_index_one() {
        let converter = ColorConverterImpl::new(vec![Red, Green, Blue]);

        assert_eq!(Red, converter.convert(1.0));
    }

    #[test]
    fn convert_index_above_one() {
        let converter = ColorConverterImpl::new(vec![Red, Green, Blue]);

        assert_eq!(Green, converter.convert(1.5));
    }
}

use crossterm::style::Color;

pub trait ColorConverter {
    fn convert(&self, level: f32) -> Color;
}

pub struct ColorConverterImpl {
    colors: Vec<Color>,
}

impl ColorConverterImpl {
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

        assert!(matches!(converter.convert(-0.2), Blue));
    }

    #[test]
    fn convert_index_zero() {
        let converter = ColorConverterImpl::new(vec![Red, Green, Blue]);

        assert!(matches!(converter.convert(0.0), Red));
    }

    #[test]
    fn convert() {
        let converter = ColorConverterImpl::new(vec![Red, Green, Blue]);

        assert!(matches!(converter.convert(0.5), Green));
    }

    #[test]
    fn convert_index_one() {
        let converter = ColorConverterImpl::new(vec![Red, Green, Blue]);

        assert!(matches!(converter.convert(1.0), Red));
    }

    #[test]
    fn convert_index_above_one() {
        let converter = ColorConverterImpl::new(vec![Red, Green, Blue]);

        assert!(matches!(converter.convert(1.5), Green));
    }
}

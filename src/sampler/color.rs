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
        assert!(level >= 0.0);
        assert!(level < 1.0);

        let index = (level * self.colors.len() as f32) as usize;

        self.colors[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crossterm::style::Color::*;

    #[test]
    #[should_panic]
    fn convert_index_below_zero() {
        ColorConverterImpl::new(vec![Red, Green, Blue]).convert(-0.1);
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
    #[should_panic]
    fn convert_index_one() {
        ColorConverterImpl::new(vec![Red, Green, Blue]).convert(1.0);
    }

    #[test]
    #[should_panic]
    fn convert_index_above_one() {
        ColorConverterImpl::new(vec![Red, Green, Blue]).convert(1.1);
    }
}

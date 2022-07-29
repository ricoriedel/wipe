pub enum Level {
    Keep,
    Draw(f32),
    Clear,
}

pub trait LevelConverter {
    fn convert(&self, level: f32) -> Level;
}

#[derive(Default)]
pub struct LevelConverterImpl;

impl LevelConverter for LevelConverterImpl {
    fn convert(&self, level: f32) -> Level {
        if level < 0.0 {
            Level::Keep
        } else if level < 1.0 {
            Level::Draw(level)
        } else {
            Level::Clear
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_keep() {
        let converter = LevelConverterImpl::default();

        assert!(matches!(converter.convert(-0.1), Level::Keep));
    }

    #[test]
    fn convert_draw() {
        let converter = LevelConverterImpl::default();

        if let Level::Draw(level) = converter.convert(0.0) {
            assert_eq!(0.0, level);
        } else {
            panic!();
        }
        if let Level::Draw(level) = converter.convert(0.5) {
            assert_eq!(0.5, level);
        } else {
            panic!();
        }
        if let Level::Draw(level) = converter.convert(0.9) {
            assert_eq!(0.9, level);
        } else {
            panic!();
        }
    }

    #[test]
    fn convert_clear() {
        let converter = LevelConverterImpl::default();

        assert!(matches!(converter.convert(1.0), Level::Clear));
        assert!(matches!(converter.convert(1.5), Level::Clear));
    }
}

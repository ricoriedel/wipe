#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Level {
    Keep,
    Draw(f32),
    Clear,
}

#[cfg_attr(test, mockall::automock)]
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

        assert_eq!(Level::Keep, converter.convert(-0.1));
    }

    #[test]
    fn convert_draw() {
        let converter = LevelConverterImpl::default();

        assert_eq!(Level::Draw(0.0), converter.convert(0.0));
        assert_eq!(Level::Draw(0.5), converter.convert(0.5));
        assert_eq!(Level::Draw(0.9), converter.convert(0.9));
    }

    #[test]
    fn convert_clear() {
        let converter = LevelConverterImpl::default();

        assert_eq!(Level::Clear, converter.convert(1.0));
        assert_eq!(Level::Clear, converter.convert(1.5));
    }
}

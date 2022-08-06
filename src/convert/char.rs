/// A sample for a terminal cell.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CharSample {
    /// Keep the char.
    Keep,
    /// Override the char.
    Draw(char),
    /// Clear the char.
    Clear,
}

/// A trait to convert a sample to a [CharSample].
#[cfg_attr(test, mockall::automock)]
pub trait CharConverter {
    fn convert(&self, level: f32) -> CharSample;
}

/// The implementation of [CharConverter].
pub struct CharConverterImpl {
    chars: Vec<char>,
}

impl CharConverterImpl {
    /// The chars used for mapping.
    pub fn new(chars: String) -> Self {
        let chars = chars.chars().collect();

        Self { chars }
    }
}

impl CharConverter for CharConverterImpl {
    fn convert(&self, level: f32) -> CharSample {
        if level < 0.0 {
            CharSample::Clear
        } else if level < 1.0 {
            let len = self.chars.len() as f32;
            let index = (level * len) as usize;

            CharSample::Draw(self.chars[index])
        } else {
            CharSample::Keep
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_clear() {
        let converter = CharConverterImpl::new("abc".to_string());

        assert_eq!(CharSample::Clear, converter.convert(-0.1));
    }

    #[test]
    fn convert_draw() {
        let converter = CharConverterImpl::new("xyz".to_string());

        assert_eq!(CharSample::Draw('x'), converter.convert(0.0));
        assert_eq!(CharSample::Draw('y'), converter.convert(0.5));
        assert_eq!(CharSample::Draw('z'), converter.convert(0.9));
    }

    #[test]
    fn convert_keep() {
        let converter = CharConverterImpl::new("123".to_string());

        assert_eq!(CharSample::Keep, converter.convert(1.0));
        assert_eq!(CharSample::Keep, converter.convert(1.5));
    }
}

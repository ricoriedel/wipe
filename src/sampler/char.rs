pub trait CharConverter {
    fn convert(&self, level: f32) -> char;
}

pub struct CharConverterImpl {
    chars: String,
    count: usize,
}

impl CharConverterImpl {
    pub fn new(chars: String) -> Self {
        let count = chars.chars().count();

        Self { chars, count }
    }
}

impl CharConverter for CharConverterImpl {
    fn convert(&self, level: f32) -> char {
        let len = self.count as f32;
        let index = (level * len).rem_euclid(len) as usize;

        self.chars.chars().nth(index).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_negative_index() {
        let converter = CharConverterImpl::new("abc".to_string());

        assert_eq!('c', converter.convert(-0.2));
    }

    #[test]
    fn convert_index_zero() {
        let converter = CharConverterImpl::new("abc".to_string());

        assert_eq!('a', converter.convert(0.0));
    }

    #[test]
    fn convert() {
        let converter = CharConverterImpl::new("abc".to_string());

        assert_eq!('b', converter.convert(0.5));
    }

    #[test]
    fn convert_index_one() {
        let converter = CharConverterImpl::new("abc".to_string());

        assert_eq!('a', converter.convert(1.0));
    }

    #[test]
    fn convert_index_above_one() {
        let converter = CharConverterImpl::new("abc".to_string());

        assert_eq!('b', converter.convert(1.5));
    }
}

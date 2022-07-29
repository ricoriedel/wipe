pub trait CharConverter {
    fn convert(&self, level: f32) -> char;
}

pub struct CharConverterImpl {
    chars: String,
}

impl CharConverterImpl {
    pub fn new(chars: String) -> Self {
        Self { chars }
    }
}

impl CharConverter for CharConverterImpl {
    fn convert(&self, level: f32) -> char {
        assert!(level >= 0.0);
        assert!(level < 1.0);

        let index = (level * self.chars.len() as f32) as usize;

        self.chars.chars().nth(index).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn convert_index_below_zero() {
        CharConverterImpl::new("abc".to_string()).convert(-0.1);
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
    #[should_panic]
    fn convert_index_one() {
        CharConverterImpl::new("abc".to_string()).convert(1.0);
    }

    #[test]
    #[should_panic]
    fn convert_index_above_one() {
        CharConverterImpl::new("abc".to_string()).convert(1.1);
    }
}

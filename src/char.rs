#[cfg(test)]
use mockall::automock;

/// Used to get a character with a given brightness.
#[cfg_attr(test, automock)]
pub trait CharSampler {
    /// Gets a character with the given brightness.
    /// # Arguments
    /// * `level`: `0 <= level` and `level < 1`
    fn sample(&self, level: f32) -> char;
}

pub struct SimpleCharSampler {
    len: f32,
    chars: String
}

impl SimpleCharSampler {
    /// # Arguments
    /// * `chars`: The characters ordered by brightness.
    pub fn new(chars: String) -> Self {
        let len = chars.chars().count() as f32;

        Self { chars, len }
    }
}

impl CharSampler for SimpleCharSampler {
    fn sample(&self, level: f32) -> char {
        assert!(0.0 <= level && level < 1.0);

        let index = level * self.len;

        self.chars.chars().nth(index as usize).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let sampler = SimpleCharSampler::new("abc".to_string());

        assert_eq!('a', sampler.sample(0.1));
        assert_eq!('b', sampler.sample(0.4));
        assert_eq!('c', sampler.sample(0.7));
    }

    #[test]
    #[should_panic]
    fn sample_index_negative() {
        SimpleCharSampler::new("abc".to_string()).sample(-0.1);
    }

    #[test]
    #[should_panic]
    fn sample_index_equals_one() {
        SimpleCharSampler::new("abc".to_string()).sample(1.0);
    }
}
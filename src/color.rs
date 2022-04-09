use crossterm::style::Color;
use mockall::automock;

/// A collection of colors.
#[automock]
pub trait ColorSampler {
    /// Gets a color for the given fill.
    /// # Arguments
    /// * `fill`: `0 <= fill` and `fill < 1`
    fn sample(&self, fill: f32) -> Color;
}

pub struct SimpleColorSampler {
    values: Vec<Color>
}

impl SimpleColorSampler {
    pub fn new(values: Vec<Color>) -> Self {
        Self { values }
    }
}

impl ColorSampler for SimpleColorSampler {
    fn sample(&self, fill: f32) -> Color {
        let index = self.values.len() as f32 * fill;

        assert!(index >= 0.0);

        self.values[index as usize]
    }
}

#[cfg(test)]
mod test {
    use crossterm::style::Color::*;
    use super::*;

    #[test]
    fn sample() {
        let sampler = SimpleColorSampler::new(vec![Red, Yellow, Green]);

        assert_eq!(Red, sampler.sample(0.1));
        assert_eq!(Yellow, sampler.sample(0.4));
        assert_eq!(Green, sampler.sample(0.7));
    }

    #[test]
    #[should_panic]
    fn sample_index_negative() {
        SimpleColorSampler::new(Vec::new()).sample(-0.1);
    }

    #[test]
    #[should_panic]
    fn sample_index_equals_one() {
        SimpleColorSampler::new(Vec::new()).sample(1.0);
    }
}
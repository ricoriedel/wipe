use clap::ArgEnum;
use crossterm::style::Color::*;
use rand::Rng;
use rand::prelude::IteratorRandom;
use crate::color::{ColorSampler, SimpleColorSampler};

#[derive(Copy, Clone, ArgEnum)]
pub enum ColorEnum {
    Red,
    Green,
    Blue,
    LightRed,
    LightGreen,
    LightBlue,
    White,
    Rainbow
}

pub trait ColorSamplerFactory {
    /// Creates the requested color.
    /// # Arguments
    /// * `color`: The color type.
    fn create(&self, colors: ColorEnum) -> Box<dyn ColorSampler>;

    /// Chooses a random color color.
    /// If none is provided, a random one of all available is chosen.
    /// # Arguments
    /// * `options`: A list of all options.
    /// * `rng`: The number generator.
    fn choose(&self, options: Vec<ColorEnum>, rng: &mut impl Rng) -> ColorEnum;
}

pub struct SimpleColorSamplerFactory;

impl SimpleColorSamplerFactory {
    pub fn new() -> Self {
        Self { }
    }
}

impl ColorSamplerFactory for SimpleColorSamplerFactory {
    fn create(&self, colors: ColorEnum) -> Box<dyn ColorSampler> {
        match colors {
            ColorEnum::Red => Box::new(SimpleColorSampler::new(vec![Yellow, DarkYellow, Red])),
            ColorEnum::Green => Box::new(SimpleColorSampler::new(vec![Cyan, DarkGreen, Green])),
            ColorEnum::Blue => Box::new(SimpleColorSampler::new(vec![Magenta, DarkBlue, Blue])),
            ColorEnum::LightRed => Box::new(SimpleColorSampler::new(vec![White, Yellow, Red])),
            ColorEnum::LightGreen => Box::new(SimpleColorSampler::new(vec![White, Cyan, Green])),
            ColorEnum::LightBlue => Box::new(SimpleColorSampler::new(vec![White, Blue, Magenta])),
            ColorEnum::White => Box::new(SimpleColorSampler::new(vec![Black, Grey, White])),
            ColorEnum::Rainbow => Box::new(SimpleColorSampler::new(vec![Magenta, Blue, Green, Yellow, Red]))
        }
    }

    fn choose(&self, mut options: Vec<ColorEnum>, rng: &mut impl Rng) -> ColorEnum {
        if options.is_empty() {
            options.push(ColorEnum::Red);
            options.push(ColorEnum::Green);
            options.push(ColorEnum::Blue);
            options.push(ColorEnum::LightRed);
            options.push(ColorEnum::LightGreen);
            options.push(ColorEnum::LightBlue);
            options.push(ColorEnum::White);
            options.push(ColorEnum::Rainbow);
        }
        options.into_iter().choose_stable(rng).unwrap()
    }
}

#[cfg(test)]
mod test {
    use rand::rngs::mock::StepRng;
    use super::*;

    #[test]
    fn create() {
        let factory = SimpleColorSamplerFactory::new();

        factory.create(ColorEnum::Red);
        factory.create(ColorEnum::Green);
        factory.create(ColorEnum::Blue);
        factory.create(ColorEnum::LightRed);
        factory.create(ColorEnum::LightGreen);
        factory.create(ColorEnum::LightBlue);
        factory.create(ColorEnum::White);
        factory.create(ColorEnum::Rainbow);
    }

    #[test]
    fn choose_from_all() {
        let rng = &mut StepRng::new(0, 1);

        let factory = SimpleColorSamplerFactory::new();

        assert!(matches!(factory.choose(Vec::new(), rng), ColorEnum::Rainbow));
    }

    #[test]
    fn choose_from_options() {
        let rng = &mut StepRng::new(0, 1);
        let options = vec![ColorEnum::Blue];

        let factory = SimpleColorSamplerFactory::new();

        assert!(matches!(factory.choose(options, rng), ColorEnum::Blue));
    }

    #[test]
    fn enum_copy() {
        let color = ColorEnum::Green;
        let copy = color;

        assert!(matches!(copy, ColorEnum::Green))
    }

    #[test]
    fn enum_clone() {
        let color = ColorEnum::Green;
        let copy = color.clone();

        assert!(matches!(copy, ColorEnum::Green))
    }
}
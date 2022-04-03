use crossterm::style::Color;
use crossterm::style::Color::*;
use rand::prelude::IteratorRandom;
use rand::Rng;
use clap::ArgEnum;

/// A collection of colors.
pub struct Pallet {
    values: Vec<Color>
}

impl Pallet {
    pub fn new(values: Vec<Color>) -> Self {
        Self { values }
    }

    pub fn red() -> Pallet {
        Pallet::new(vec![Yellow, DarkYellow, Red])
    }

    pub fn red_light() -> Pallet {
        Pallet::new(vec![White, Yellow, Red])
    }

    pub fn green() -> Pallet {
        Pallet::new(vec![Cyan, DarkGreen, Green])
    }

    pub fn green_light() -> Pallet {
        Pallet::new(vec![White, Cyan, Green])
    }

    pub fn blue() -> Pallet {
        Pallet::new(vec![Magenta, DarkBlue, Blue])
    }

    pub fn blue_light() -> Pallet {
        Pallet::new(vec![White, Magenta, Blue])
    }

    pub fn white() -> Pallet {
        Pallet::new(vec![Black, Grey, White])
    }

    pub fn rainbow() -> Pallet {
        Pallet::new(vec![Magenta, Blue, Green, Yellow, Red])
    }

    /// Gets a color for the given fill.
    /// # Arguments
    /// * `fill`: `0 <= fill` and `fill < 1`
    pub fn sample(&self, fill: f32) -> Color {
        let pos = self.values.len() as f32 * fill;
        let index = pos as usize;

        self.values[index]
    }
}

#[derive(Copy, Clone, ArgEnum)]
pub enum PalletEnum {
    Red,
    RedLight,
    Green,
    GreenLight,
    Blue,
    BlueLight,
    White,
    Rainbow
}


/// Chooses a random color pallet.
/// If none is provided, a random one of all available is chosen.
/// # Arguments
/// * `options`: A list of all options.
/// * `rng`: The number generator.
pub fn choose_pallet(mut options: Vec<PalletEnum>, rng: &mut impl Rng) -> PalletEnum {
    if options.is_empty() {
        options.push(PalletEnum::Red);
        options.push(PalletEnum::RedLight);
        options.push(PalletEnum::Green);
        options.push(PalletEnum::GreenLight);
        options.push(PalletEnum::Blue);
        options.push(PalletEnum::BlueLight);
        options.push(PalletEnum::White);
        options.push(PalletEnum::Rainbow);
    }
    options.into_iter().choose(rng).unwrap()
}

/// Creates the requested pallet.
/// # Arguments
/// * `pallet`: The pallet type.
pub fn create_pallet(pallet: PalletEnum) -> Pallet {
    match pallet {
        PalletEnum::Red => Pallet::red(),
        PalletEnum::RedLight => Pallet::red_light(),
        PalletEnum::Green => Pallet::green(),
        PalletEnum::GreenLight => Pallet::green_light(),
        PalletEnum::Blue => Pallet::blue(),
        PalletEnum::BlueLight => Pallet::blue_light(),
        PalletEnum::White => Pallet::white(),
        PalletEnum::Rainbow => Pallet::rainbow()
    }
}
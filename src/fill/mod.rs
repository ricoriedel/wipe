mod level;

use clap::ArgEnum;
use rand::prelude::IteratorRandom;
use rand::Rng;
use crate::fill::level::LevelFillMode;
use crate::vec::Vector;

pub trait FillMode {
    fn sample(&self, level: f32, pos: Vector) -> f32;
}

#[derive(Copy, Clone, ArgEnum)]
pub enum FillModeEnum {
    Circle,
    Level,
    Stripe
}

pub fn choose_fill_mode(mut options: Vec<FillModeEnum>, rng: &mut impl Rng) -> FillModeEnum {
    if options.is_empty() {
        options.push(FillModeEnum::Circle);
        options.push(FillModeEnum::Level);
        options.push(FillModeEnum::Stripe);
    }
    options.into_iter().choose(rng).unwrap()
}

pub fn create_fill_mode(mode: FillModeEnum, _: Vector) -> Box<dyn FillMode> {
    match mode {
        FillModeEnum::Circle => todo!(),
        FillModeEnum::Level => Box::new(LevelFillMode::new()),
        FillModeEnum::Stripe => todo!()
    }
}
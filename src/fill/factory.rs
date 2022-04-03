use clap::ArgEnum;
use rand::prelude::IteratorRandom;
use rand::Rng;
use crate::fill::FillMode;
use crate::fill::level::LevelFillMode;
use crate::vec::Vector;

#[derive(Copy, Clone, ArgEnum)]
pub enum FillModeEnum {
    Circle,
    Level,
    Stripe
}

pub trait FillModeFactory {
    fn create(&self, mode: FillModeEnum, size: Vector) -> Box<dyn FillMode>;

    fn choose(&self, options: Vec<FillModeEnum>, rng: &mut impl Rng) -> FillModeEnum;
}

pub struct SimpleFillModeFactory;

impl SimpleFillModeFactory {
    pub fn new() -> Self {
        Self { }
    }
}

impl FillModeFactory for SimpleFillModeFactory {
    fn create(&self, mode: FillModeEnum, _: Vector) -> Box<dyn FillMode> {
        match mode {
            FillModeEnum::Circle => todo!(),
            FillModeEnum::Level => Box::new(LevelFillMode::new()),
            FillModeEnum::Stripe => todo!()
        }
    }

    fn choose(&self, mut options: Vec<FillModeEnum>, rng: &mut impl Rng) -> FillModeEnum {
        if options.is_empty() {
            options.push(FillModeEnum::Circle);
            options.push(FillModeEnum::Level);
            options.push(FillModeEnum::Stripe);
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
        let factory = SimpleFillModeFactory::new();
        let size = Vector::ZERO;

        factory.create(FillModeEnum::Circle, size);
        factory.create(FillModeEnum::Level, size);
        factory.create(FillModeEnum::Stripe, size);
    }

    #[test]
    fn choose_from_all() {
        let rng = &mut StepRng::new(0, 1);

        let factory = SimpleFillModeFactory::new();

        assert!(matches!(factory.choose(Vec::new(), rng), FillModeEnum::Stripe));
    }

    #[test]
    fn choose_from_options() {
        let rng = &mut StepRng::new(0, 1);
        let options = vec![FillModeEnum::Circle];

        let factory = SimpleFillModeFactory::new();

        assert!(matches!(factory.choose(options, rng), FillModeEnum::Circle));
    }
}
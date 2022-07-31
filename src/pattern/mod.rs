mod circle;
mod line;
mod rhombus;
mod wheel;

use crate::vec::Vector;

#[derive(Copy, Clone)]
pub struct Config {
    pub size: Vector,
    pub step: f32,
}

pub trait PatternFactory {
    fn name(&self) -> String;

    fn create(&self, config: &Config) -> Box<dyn Pattern>;
}

#[cfg_attr(test, mockall::automock)]
pub trait Pattern {
    fn sample(&self, pos: Vector) -> f32;
}

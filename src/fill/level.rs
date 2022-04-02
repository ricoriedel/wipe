use crate::fill::FillMode;
use crate::vec::Vector;

pub struct LevelFillMode;

impl LevelFillMode {
    pub fn new() -> Self {
        Self { }
    }
}

impl FillMode for LevelFillMode {
    fn sample(&self, level: f32, _: Vector) -> f32 {
        level
    }
}
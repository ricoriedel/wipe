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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let mode = LevelFillMode::new();

        assert_eq!(0.3, mode.sample(0.3, Vector::ZERO));
        assert_eq!(0.7, mode.sample(0.7, Vector::new(0.1, 0.2)));
    }
}
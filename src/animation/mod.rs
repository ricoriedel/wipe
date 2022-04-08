mod circle;

use crate::vec::Vector;

pub trait Animation {
    fn sample(&self, step: f32, pos: Vector) -> f32;
}
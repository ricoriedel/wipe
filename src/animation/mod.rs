pub mod circle;
pub mod rotation;
pub mod rhombus;

use crate::vec::Vector;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Animation {
    fn sample(&self, step: f32, pos: Vector) -> f32;
}
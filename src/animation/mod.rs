pub mod circle;
pub mod rotation;
pub mod rhombus;

use crate::vec::Vector;

use mockall::automock;

#[automock]
pub trait Animation {
    fn sample(&self, step: f32, pos: Vector) -> f32;
}
pub mod circle;

use crate::vec::Vector;

use mockall::automock;

#[automock]
pub trait Animation {
    fn sample(&self, step: f32, pos: Vector) -> f32;
}
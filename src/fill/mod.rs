pub mod level;
pub mod circle;
pub mod stripes;

use crate::vec::Vector;

use mockall::automock;

/// Used to choose the colors of characters.
#[automock]
pub trait FillMode {
    /// Gets the color for this character.
    fn sample(&self, level: f32, pos: Vector) -> f32;
}
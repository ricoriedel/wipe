mod level;
mod circle;

use crate::vec::Vector;

/// Used to choose the colors of characters.
pub trait FillMode {
    /// Gets the color for this character.
    fn sample(&self, level: f32, pos: Vector) -> f32;
}
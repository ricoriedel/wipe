pub mod level;
pub mod circle;
pub mod stripes;

use crate::vec::Vector;

#[cfg(test)]
use mockall::automock;

/// Used to choose the colors of characters.
#[cfg_attr(test, automock)]
pub trait FillMode {
    /// Gets the color for this character.
    /// # Arguments
    /// * `step`: `0 <= step` and `step <= 1`
    fn sample(&self, level: f32, pos: Vector) -> f32;
}
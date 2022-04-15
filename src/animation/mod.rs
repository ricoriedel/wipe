pub mod circle;
pub mod rotation;
pub mod rhombus;

use crate::vec::Vector;

#[cfg(test)]
use mockall::automock;

/// A sampler for an animation.
#[cfg_attr(test, automock)]
pub trait Animation {
    /// Returns the level (of brightness) for the
    /// given step of the animation an position on screen.
    /// # Arguments
    /// * `step`: `0 <= step` and `step <= 1`
    ///
    /// # Return values
    /// * `1 < n` => Keep current character
    /// * `0 <= n` and `n < 1` => Draw some character
    /// * `n < 0` => Clear character
    fn sample(&self, step: f32, pos: Vector) -> f32;
}
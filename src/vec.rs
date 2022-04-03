/// A vector with a x and y axis.
#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Creates a vector with the on screen coordinates based on the terminal coordinates.
    /// # Arguments
    /// * `x`: The x axis of the terminal character.
    /// * `y`: The y axis of the terminal character.
    pub fn from_terminal(x: usize, y: usize) -> Self {
        Vector::new(x as f32, y as f32 * 2.0)
    }
}
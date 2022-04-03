/// A vector with a x and y axis.
#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32
}

impl Vector {
    pub const ZERO: Vector = Vector::new(0.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Creates a vector with the on screen coordinates based on the terminal coordinates.
    /// # Arguments
    /// * `x`: The x axis of the terminal character.
    /// * `y`: The y axis of the terminal character.
    pub fn from_terminal(x: usize, y: usize) -> Self {
        Self::new(x as f32, y as f32 * 2.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let vec = Vector::new(3.0, 5.0);

        assert_eq!(3.0, vec.x);
        assert_eq!(5.0, vec.y);
    }

    #[test]
    fn from_terminal() {
        let vec = Vector::from_terminal(2, 4);

        assert_eq!(2.0, vec.x);
        assert_eq!(8.0, vec.y);
    }

    #[test]
    fn copy() {
        let vec = Vector::new(2.0, 4.0);
        let copy = vec;

        assert_eq!(vec.x, copy.x);
        assert_eq!(vec.y, copy.y);
    }

    #[test]
    fn clone() {
        let vec = Vector::new(2.0, 4.0);
        let clone = vec.clone();

        assert_eq!(vec.x, clone.x);
        assert_eq!(vec.y, clone.y);
    }
}
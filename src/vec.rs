use std::ops::Sub;

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

    pub fn center(self) -> Self {
        Self::new(self.x / 2.0, self.y / 2.0)
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn smaller(self) -> f32 {
        self.x.min(self.y)
    }

    /// Creates a vector with the on screen coordinates based on the terminal coordinates.
    /// # Arguments
    /// * `x`: The x axis of the terminal character.
    /// * `y`: The y axis of the terminal character.
    pub fn from_terminal(x: usize, y: usize) -> Self {
        Self::new(x as f32, y as f32 * 2.0)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
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
    fn center() {
        let vec = Vector::new(3.0, 8.0);

        assert_eq!(1.5, vec.center().x);
        assert_eq!(4.0, vec.center().y);
    }

    #[test]
    fn length() {
        let vec = Vector::new(3.0, 6.0);

        assert!(6.7 < vec.length() && vec.length() < 6.8);
    }

    #[test]
    fn smaller() {
        assert_eq!(4.0, Vector::new(7.0, 4.0).smaller());
        assert_eq!(2.0, Vector::new(2.0, 9.0).smaller());
    }

    #[test]
    fn from_terminal() {
        let vec = Vector::from_terminal(2, 4);

        assert_eq!(2.0, vec.x);
        assert_eq!(8.0, vec.y);
    }

    #[test]
    fn sub() {
        let left = Vector::new(8.0, 15.0);
        let right = Vector::new(2.0, 4.0);
        let result = left - right;

        assert_eq!(6.0, result.x);
        assert_eq!(11.0, result.y);
    }
}
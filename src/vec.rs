/// A vector with a x-axis and y-axis.
#[derive(Copy, Clone, PartialEq, Debug, Default, derive_more::Sub)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    /// Creates a new vector.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Creates a new vector from terminal coordinates.
    pub fn from_terminal(x: u16, y: u16) -> Self {
        Vector::new(x as f32, y as f32 * 2.0)
    }

    /// Returns the length.
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns a vector with absolute values.
    pub fn abs(&self) -> Vector {
        Self::new(self.x.abs(), self.y.abs())
    }

    /// Returns the sum of all axis.
    pub fn sum(&self) -> f32 {
        self.x + self.y
    }

    /// Returns the center.
    pub fn center(&self) -> Vector {
        Self::new(self.x / 2.0, self.y / 2.0)
    }

    /// Returns the angle.
    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    /// Returns a vector with x and y swapped.
    pub fn swap(&self) -> Vector {
        Self::new(self.y, self.x)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn new() {
        let v = Vector::new(4.0, 7.0);
        assert_eq!(4.0, v.x);
        assert_eq!(7.0, v.y);
    }

    #[test]
    fn len() {
        assert_abs_diff_eq!(8.5, Vector::new(3.0, 8.0).len(), epsilon = 0.1);
    }

    #[test]
    fn abs() {
        assert_eq!(Vector::new(3.0, 7.0), Vector::new(-3.0, -7.0).abs());
    }

    #[test]
    fn sum() {
        assert_eq!(11.0, Vector::new(3.0, 8.0).sum());
    }

    #[test]
    fn center() {
        assert_eq!(Vector::new(4.0, 9.0), Vector::new(8.0, 18.0).center());
    }

    #[test]
    fn angle() {
        assert_abs_diff_eq!(-1.5, Vector::new(2.0, -20.0).angle(), epsilon = 0.1);
    }

    #[test]
    fn swap() {
        assert_eq!(Vector::new(7.0, 2.0), Vector::new(2.0, 7.0).swap());
    }
}

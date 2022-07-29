use std::ops::Sub;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn center(&self) -> Vector {
        Self::new(self.x / 2.0, self.y / 2.0)
    }

    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
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
    fn center() {
        assert_eq!(Vector::new(4.0, 9.0), Vector::new(8.0, 18.0).center());
    }

    #[test]
    fn angle() {
        assert_abs_diff_eq!(-1.5, Vector::new(2.0, -20.0).angle(), epsilon = 0.1);
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vector::new(-5.0, 10.0),
            Vector::new(3.0, 16.0) - Vector::new(8.0, 6.0)
        );
    }
}

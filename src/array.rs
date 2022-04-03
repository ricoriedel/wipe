use std::ops::{Index, IndexMut};

/// A two dimensional statically size array.
pub struct Array2D<T> {
    width: usize,
    height: usize,
    values: Vec<T>
}

impl<T: Default + Copy> Array2D<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            values: vec![T::default(); width * height]
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Calculates the physical index of the given position.
    ///
    /// # Panics
    /// Panics if the position is out of bounds.
    fn index_of(&self, pos: (usize, usize)) -> usize {
        assert!(pos.0 < self.width);
        assert!(pos.1 < self.height);
        pos.0 + pos.1 * self.width
    }
}

impl<T: Default + Copy> Index<(usize, usize)> for Array2D<T> {
    type Output = T;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        unsafe { self.values.get_unchecked(self.index_of(pos)) }
    }
}

impl<T: Default + Copy> IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        let i = self.index_of(pos);

        unsafe { self.values.get_unchecked_mut(i) }
    }
}

#[cfg(test)]
mod test {
    use crate::array::Array2D;

    #[test]
    fn width() {
        let array = Array2D::<()>::new(10, 4);

        assert_eq!(10, array.width());
    }

    #[test]
    fn height() {
        let array = Array2D::<()>::new(2, 5);

        assert_eq!(5, array.height());
    }

    #[test]
    fn index() {
        let mut array = Array2D::new(4, 4);

        array[(1, 2)] = 3;
        array[(3, 3)] = 7;

        assert_eq!(3, array[(1, 2)]);
        assert_eq!(7, array[(3, 3)]);
    }

    #[test]
    #[should_panic]
    fn index_oob_width() {
        let array = Array2D::<()>::new(5, 10);

        array[(8, 2)];
    }

    #[test]
    #[should_panic]
    fn index_oob_height() {
        let array = Array2D::<()>::new(10, 5);

        array[(3, 7)];
    }
}
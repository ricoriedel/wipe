use crate::FillMode;
use crate::vec::Vector;

const INTERVAL: f32 = 4.0;

/// Fill based on diagonal stripes.
pub struct StripesFillMode {
    interval: f32
}

impl StripesFillMode {
    pub fn new(size: Vector) -> Self {
        Self {
            interval: size.smaller() / INTERVAL
        }
    }
}

impl FillMode for StripesFillMode {
    fn sample(&self, _: f32, pos: Vector) -> f32 {
        (pos.sum() % self.interval) / self.interval
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let mode = StripesFillMode::new(Vector::new(8.0, 4.0));

        assert_eq!(0.25, mode.sample(0.0, Vector::new(1.5, 0.75)));
        assert_eq!(0.5, mode.sample(0.0, Vector::new(4.0, 2.5)));
    }
}
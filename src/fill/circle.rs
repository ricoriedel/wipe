use crate::fill::FillMode;
use crate::vec::Vector;

const INTERVAL: f32 = 4.0;

/// Fill based on rings of a circle.
pub struct CircleFillMode {
    center: Vector,
    interval: f32
}

impl CircleFillMode {
    pub fn new(size: Vector) -> Self {
        Self {
            center: size.center(),
            interval: size.smaller() / INTERVAL,
        }
    }
}

impl FillMode for CircleFillMode {
    fn sample(&self, _: f32, pos: Vector) -> f32 {
        ((pos - self.center).length() % self.interval) / self.interval
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let fill = CircleFillMode::new(Vector::new(10.0, 8.0));

        let sample_1 = fill.sample(0.0, Vector::new(5.0, 3.0));
        let sample_2 = fill.sample(0.0, Vector::new(8.5, 4.0));

        assert!(0.4 < sample_1 && sample_1 < 0.6);
        assert!(0.7 < sample_2 && sample_2 < 0.8);
    }
}
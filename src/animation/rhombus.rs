use crate::animation::Animation;
use crate::vec::Vector;

const THICKNESS: f32 = 0.2;
const FINAL_DISTANCE: f32 = 1.0 + THICKNESS * 2.0;

/// An animation of an expanding rhombus.
pub struct RhombusAnimation {
    center: Vector,
    thickness: f32,
    final_distance: f32,
}

impl RhombusAnimation {
    pub fn new(size: Vector) -> Self {
        let center = size.center();
        let distance = center.sum();

        Self {
            center,
            thickness: distance * THICKNESS,
            final_distance: distance * FINAL_DISTANCE,
        }
    }
}

impl Animation for RhombusAnimation {
    fn sample(&self, step: f32, pos: Vector) -> f32 {
        let dist = self.final_distance * step - self.thickness;
        let pos_dist = (self.center - pos).abs().sum();

        (pos_dist - dist) / self.thickness
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let anim = RhombusAnimation::new(Vector::new(30.0, 10.0));

        let sample_1 = anim.sample(0.2, Vector::new(5.0, 16.0));
        let sample_2 = anim.sample(0.7, Vector::new(22.0, 2.0));
        let sample_3 = anim.sample(0.5, Vector::new(4.0, 7.0));

        assert!(4.8 < sample_1 && sample_1 < 4.9);
        assert!(-1.5 < sample_2 && sample_2 < -1.4);
        assert!(0.7 < sample_3 && sample_3 < 0.8);
    }
}
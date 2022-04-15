use std::f32::consts::PI;
use crate::animation::Animation;
use crate::vec::Vector;

const TWO_PI: f32 = PI * 2.0;
const THICKNESS: f32 = TWO_PI * 0.1;
const FULL_ROTATION: f32 = TWO_PI + THICKNESS * 2.0;

pub struct RotationAnimation {
    center: Vector
}

impl RotationAnimation {
    pub fn new(size: Vector) -> Self {
        Self {
            center: size.center()
        }
    }
}

impl Animation for RotationAnimation {
    fn sample(&self, step: f32, pos: Vector) -> f32 {
        let angle = FULL_ROTATION * step - PI - THICKNESS;
        let pos_angle = (pos - self.center).angle();

        (pos_angle - angle) / THICKNESS
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let anim = RotationAnimation::new(Vector::new(30.0, 10.0));

        let sample_1 = anim.sample(0.3, Vector::new(16.0, 3.0));
        let sample_2 = anim.sample(0.7, Vector::new(22.0, 2.0));
        let sample_3 = anim.sample(0.5, Vector::new(4.0, 7.0));

        assert!(0.6 < sample_1 && sample_1 < 0.7);
        assert!(-3.1 < sample_2 && sample_2 < -3.0);
        assert!(4.7 < sample_3 && sample_3 < 4.8);
    }
}
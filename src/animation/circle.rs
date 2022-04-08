use crate::animation::Animation;
use crate::vec::Vector;

const THICKNESS: f32 = 0.2;
const FINAL_RADIUS: f32 = 1.0 + THICKNESS * 2.0;

pub struct CircleAnimation {
    center: Vector,
    thickness: f32,
    final_radius: f32,
}

impl CircleAnimation {
    pub fn new(size: Vector) -> Self {
        let center = size.center();
        let distance = center.length();

        Self {
            center,
            thickness: distance * THICKNESS,
            final_radius: distance * FINAL_RADIUS,
        }
    }
}

impl Animation for CircleAnimation {
    fn sample(&self, step: f32, pos: Vector) -> f32 {
        let radius = self.final_radius * step - self.thickness;
        let distance = (pos - self.center).length();

        (distance - radius) / self.thickness
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let anim = CircleAnimation::new(Vector::new(10.0, 20.0));

        let sample_1 = anim.sample(0.5, Vector::new(17.0, 5.0));
        let sample_2 = anim.sample(0.8, Vector::new(11.0, 8.0));
        let sample_3 = anim.sample(0.2, Vector::new(7.0, 10.0));

        assert!(3.3 < sample_1 && sample_1 < 3.4);
        assert!(-1.8 < sample_2 && sample_2 < -1.7);
        assert!(0.4 < sample_3 && sample_3 < 0.5);
    }
}
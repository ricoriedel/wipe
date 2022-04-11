use crossterm::style::Color;
use crate::animation::Animation;
use crate::char::CharSampler;
use crate::color::ColorSampler;
use crate::fill::FillMode;
use crate::vec::Vector;

#[cfg(test)]
use mockall::automock;

pub enum Sample {
    Keep,
    Draw { char: char, color: Color },
    Clear,
}

#[cfg_attr(test, automock)]
pub trait Sampler {
    fn sample(&self, step: f32, pos: Vector) -> Sample;
}

pub struct ComposedSampler {
    animation: Box<dyn Animation>,
    fill: Box<dyn FillMode>,
    color: Box<dyn ColorSampler>,
    char: Box<dyn CharSampler>,
}

impl ComposedSampler {
    pub fn new(animation: Box<dyn Animation>,
               fill: Box<dyn FillMode>,
               color: Box<dyn ColorSampler>,
               char: Box<dyn CharSampler>) -> Self {
        Self { animation, fill, color, char }
    }
}

impl Sampler for ComposedSampler {
    fn sample(&self, step: f32, pos: Vector) -> Sample {
        let level = self.animation.sample(step, pos);

        if level >= 1.0 {
            Sample::Keep
        } else if level >= 0.0 {
            let char = self.char.sample(level);
            let fill = self.fill.sample(level, pos);
            let color = self.color.sample(fill);

            Sample::Draw { char, color }
        } else {
            Sample::Clear
        }
    }
}

#[cfg(test)]
mod test {
    use mockall::predicate::{always, eq};
    use super::*;
    use crate::animation::MockAnimation;
    use crate::fill::MockFillMode;
    use crate::color::MockColorSampler;
    use crate::char::MockCharSampler;

    #[test]
    fn sample_keep() {
        let mut anim = Box::new(MockAnimation::new());
        let fill = Box::new(MockFillMode::new());
        let color = Box::new(MockColorSampler::new());
        let char = Box::new(MockCharSampler::new());

        anim.expect_sample().return_const(3.0);

        let sampler = ComposedSampler::new(anim, fill, color, char);

        assert!(matches!(sampler.sample(0.7, Vector::new(0.3, 0.1)), Sample::Keep));
    }

    #[test]
    fn sample_draw() {
        let mut anim = Box::new(MockAnimation::new());
        let mut fill = Box::new(MockFillMode::new());
        let mut color = Box::new(MockColorSampler::new());
        let mut char = Box::new(MockCharSampler::new());

        anim.expect_sample().once().with(eq(0.2), always()).return_const(0.3);
        fill.expect_sample().once().with(eq(0.3), always()).return_const(0.8);
        color.expect_sample().once().with(eq(0.8)).return_const(Color::Blue);
        char.expect_sample().once().with(eq(0.3)).return_const('Z');

        let sampler = ComposedSampler::new(anim, fill, color, char);

        assert!(matches!(sampler.sample(0.2, Vector::new(0.3, 0.1)), Sample::Draw { char: 'Z', color: Color::Blue }));
    }

    #[test]
    fn sample_clear() {
        let mut anim = Box::new(MockAnimation::new());
        let fill = Box::new(MockFillMode::new());
        let color = Box::new(MockColorSampler::new());
        let char = Box::new(MockCharSampler::new());

        anim.expect_sample().return_const(-0.4);

        let sampler = ComposedSampler::new(anim, fill, color, char);

        assert!(matches!(sampler.sample(0.7, Vector::new(0.3, 0.1)), Sample::Clear));
    }

    #[test]
    fn sample_almost_draw() {
        let mut anim = Box::new(MockAnimation::new());
        let fill = Box::new(MockFillMode::new());
        let color = Box::new(MockColorSampler::new());
        let char = Box::new(MockCharSampler::new());

        anim.expect_sample().return_const(1.0);

        let sampler = ComposedSampler::new(anim, fill, color, char);

        assert!(matches!(sampler.sample(0.7, Vector::new(0.3, 0.1)), Sample::Keep));
    }


    #[test]
    fn sample_almost_clear() {
        let mut anim = Box::new(MockAnimation::new());
        let mut fill = Box::new(MockFillMode::new());
        let mut color = Box::new(MockColorSampler::new());
        let mut char = Box::new(MockCharSampler::new());

        anim.expect_sample().return_const(0.0);
        fill.expect_sample().return_const(0.8);
        color.expect_sample().return_const(Color::Blue);
        char.expect_sample().return_const('a');

        let sampler = ComposedSampler::new(anim, fill, color, char);

        assert!(matches!(sampler.sample(0.7, Vector::new(0.3, 0.1)), Sample::Draw { .. }));
    }
}
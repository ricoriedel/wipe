use crossterm::style::Color;
use crate::animation::Animation;
use crate::char::CharSampler;
use crate::color::ColorSampler;
use crate::fill::FillMode;
use crate::vec::Vector;

pub enum Sample {
    Keep,
    Draw { char: char, color: Color },
    Clear,
}

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

        if level > 1.0 {
            Sample::Keep
        } else if level > 0.0 {
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
    use crate::char::SimpleCharSampler;
    use super::*;

    struct MockAnimation;
    struct MockFillMode;
    struct MockColorSampler;

    impl Animation for MockAnimation {
        fn sample(&self, step: f32, pos: Vector) -> f32 {
            step + pos.x + pos.y
        }
    }
    impl FillMode for MockFillMode {
        fn sample(&self, level: f32, pos: Vector) -> f32 {
            level + pos.x + pos.y
        }
    }
    impl ColorSampler for MockColorSampler {
        fn sample(&self, _: f32) -> Color {
            Color::Green
        }
    }

    fn create_sampler() -> ComposedSampler {
        let anim = Box::new(MockAnimation { });
        let fill = Box::new(MockFillMode { });
        let color = Box::new(MockColorSampler { });
        let char = Box::new(SimpleCharSampler::new("0123456789".to_string()));

        ComposedSampler::new(anim, fill, color, char)
    }

    #[test]
    fn sample_keep() {
        assert!(matches!(create_sampler().sample(0.7, Vector::new(0.3, 0.1)), Sample::Keep));
    }

    #[test]
    fn sample_draw() {
        assert!(matches!(create_sampler().sample(0.4, Vector::new(0.1, 0.1)), Sample::Draw { char: '6', color: Color::Green }));
    }

    #[test]
    fn sample_clear() {
        assert!(matches!(create_sampler().sample(-1.0, Vector::new(0.4, 0.5)), Sample::Clear));
    }
}
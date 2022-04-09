use anyhow::Error;
use crate::sampler::{Sample, Sampler};
use crate::surface::Surface;
use crate::Vector;

pub trait Renderer {
    fn render(&mut self, step: f32);
    fn present(&mut self) -> Result<(), Error>;
    fn finish(&mut self) -> Result<(), Error>;
}

pub struct SamplerRenderer {
    surface: Box<dyn Surface>,
    sampler: Box<dyn Sampler>,
}

impl SamplerRenderer {
    pub fn new(surface: Box<dyn Surface>,
               sampler: Box<dyn Sampler>) -> Self {
        Self { surface, sampler }
    }
}

impl Renderer for SamplerRenderer {
    fn render(&mut self, step: f32) {
        for x in 0..self.surface.width() {
            for y in 0..self.surface.height() {
                let pos = Vector::from_terminal(x, y);
                let sample = self.sampler.sample(step, pos);

                match sample {
                    Sample::Keep => (),
                    Sample::Draw { char, color } => self.surface.draw(x, y, char, color),
                    Sample::Clear => self.surface.clear(x, y),
                }
            }
        }
    }

    fn present(&mut self) -> Result<(), Error> {
        self.surface.present()
    }

    fn finish(&mut self) -> Result<(), Error> {
        self.surface.finish()
    }
}

#[cfg(test)]
mod test {
    use crossterm::style::*;
    use mockall::predicate::*;
    use super::*;
    use crate::surface::MockSurface;
    use crate::sampler::MockSampler;

    #[test]
    fn render() {
        let mut surface = Box::new(MockSurface::new());
        let mut sampler = Box::new(MockSampler::new());

        sampler.expect_sample().withf(|_, pos| pos.x == 0.0 && pos.y == 0.0).returning(|_,_| Sample::Clear);
        sampler.expect_sample().withf(|_, pos| pos.x == 1.0 && pos.y == 0.0).returning(|_,_| Sample::Keep);
        sampler.expect_sample().withf(|_, pos| pos.x == 0.0 && pos.y == 2.0).returning(|_,_| Sample::Draw { char: 'a', color: Color::Red });
        sampler.expect_sample().withf(|_, pos| pos.x == 1.0 && pos.y == 2.0).returning(|_,_| Sample::Keep);
        sampler.expect_sample().withf(|_, pos| pos.x == 0.0 && pos.y == 4.0).returning(|_,_| Sample::Draw { char: 'x', color: Color::Yellow });
        sampler.expect_sample().withf(|_, pos| pos.x == 1.0 && pos.y == 4.0).returning(|_,_| Sample::Clear);

        surface.expect_width().return_const(2 as usize);
        surface.expect_height().return_const(3 as usize);
        surface.expect_clear().once().with(eq(0), eq(0)).return_const(());
        surface.expect_draw().once().with(eq(0), eq(1), eq('a'), eq(Color::Red)).return_const(());
        surface.expect_draw().once().with(eq(0), eq(2), eq('x'), eq(Color::Yellow)).return_const(());
        surface.expect_clear().once().with(eq(1), eq(2)).return_const(());

        let mut renderer = SamplerRenderer::new(surface, sampler);

        renderer.render(0.5);
    }

    #[test]
    fn present() {
        let mut surface = Box::new(MockSurface::new());
        let sampler = Box::new(MockSampler::new());

        surface.expect_present().once().returning(|| Ok(()));

        let mut renderer = SamplerRenderer::new(surface, sampler);

        renderer.present().unwrap();
    }

    #[test]
    fn finish() {
        let mut surface = Box::new(MockSurface::new());
        let sampler = Box::new(MockSampler::new());

        surface.expect_finish().once().returning(|| Ok(()));

        let mut renderer = SamplerRenderer::new(surface, sampler);

        renderer.finish().unwrap();
    }
}
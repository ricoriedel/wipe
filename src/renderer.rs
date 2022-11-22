use crate::convert::{CharSample, Converter};
use crate::pattern::*;
use crate::Error;
use crate::Printer;
use crate::Vector;
use crossterm::style::Color;

/// A renderer for an animation.
#[cfg_attr(test, mockall::automock)]
pub trait Renderer {
    /// Renders the current frame and flushes.
    fn render(&mut self, step: f32) -> Result<(), Error>;
}

/// The implementation of [Renderer].
pub struct RendererImpl<T1, T2, T3: Printer> {
    sampler: T1,
    converter: T2,
    printer: T3,
}

impl<T1, T2, T3: Printer> RendererImpl<T1, T2, T3> {
    pub fn new(sampler: T1, converter: T2, mut printer: T3) -> Result<Self, Error> {
        printer.hide_cursor()?;

        Ok(Self {
            sampler,
            converter,
            printer,
        })
    }
}

impl<T1: SamplerFactory, T2: Converter, T3: Printer> Renderer for RendererImpl<T1, T2, T3> {
    fn render(&mut self, step: f32) -> Result<(), Error> {
        let (width, height) = self.printer.size()?;
        let config = Config {
            step,
            size: Vector::from_terminal(width, height),
        };
        let sampler = self.sampler.create(&config);

        for y in 0..height {
            for x in 0..width {
                let pos = Vector::from_terminal(x, y);

                match self.converter.char(sampler.char(pos)) {
                    CharSample::Draw(char) => {
                        let color = self.converter.color(sampler.color(pos));

                        self.printer.move_to(x, y)?;
                        self.printer.set_foreground(color)?;
                        self.printer.print(char)?;
                    }
                    CharSample::Clear => {
                        self.printer.move_to(x, y)?;
                        self.printer.print(' ')?;
                    }
                    CharSample::Keep => (),
                }
            }
        }
        self.printer.flush()
    }
}

impl<T1, T2, T3: Printer> Drop for RendererImpl<T1, T2, T3> {
    fn drop(&mut self) {
        // Errors while dropping the renderer can be safely ignored.
        self.printer.move_to(0, 0).ok();
        self.printer.set_foreground(Color::Reset).ok();
        self.printer.show_cursor().ok();
        self.printer.clear().ok();
        self.printer.flush().ok();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::convert::MockConverter;
    use crate::pattern::MockSampler;
    use crate::pattern::MockSamplerFactory;
    use crate::MockPrinter;
    use crossterm::style::Color;
    use mockall::predicate::eq;
    use mockall::Sequence;

    #[test]
    fn new() {
        let factory = MockSamplerFactory::new();
        let converter = MockConverter::new();
        let mut printer = MockPrinter::new();

        // Constructor
        printer.expect_hide_cursor().once().returning(|| Ok(()));

        // Drop
        printer.expect_move_to().returning(|_, _| Ok(()));
        printer.expect_set_foreground().returning(|_| Ok(()));
        printer.expect_show_cursor().returning(|| Ok(()));
        printer.expect_clear().returning(|| Ok(()));
        printer.expect_flush().returning(|| Ok(()));

        drop(RendererImpl::new(factory, converter, printer));
    }

    #[test]
    fn render_config_correct() {
        let mut sampler = MockSamplerFactory::new();
        let mut converter = MockConverter::new();
        let mut printer = MockPrinter::new();

        printer.expect_size().returning(|| Ok((2, 2)));
        sampler.expect_create().returning(|_| {
            let mut sampler = MockSampler::new();
            sampler
                .expect_char()
                .with(eq(Vector::new(0.0, 0.0)))
                .return_const(1.0);
            sampler
                .expect_char()
                .with(eq(Vector::new(1.0, 0.0)))
                .return_const(2.0);
            sampler
                .expect_char()
                .with(eq(Vector::new(0.0, 2.0)))
                .return_const(3.0);
            sampler
                .expect_char()
                .with(eq(Vector::new(1.0, 2.0)))
                .return_const(4.0);
            sampler
                .expect_color()
                .with(eq(Vector::new(0.0, 0.0)))
                .return_const(5.0);
            sampler
                .expect_color()
                .with(eq(Vector::new(1.0, 0.0)))
                .return_const(6.0);
            sampler
                .expect_color()
                .with(eq(Vector::new(0.0, 2.0)))
                .return_const(7.0);
            sampler
                .expect_color()
                .with(eq(Vector::new(1.0, 2.0)))
                .return_const(8.0);
            sampler
        });
        converter
            .expect_char()
            .with(eq(1.0))
            .return_const(CharSample::Keep);
        converter
            .expect_char()
            .with(eq(2.0))
            .return_const(CharSample::Clear);
        converter
            .expect_char()
            .with(eq(3.0))
            .return_const(CharSample::Draw('A'));
        converter
            .expect_char()
            .with(eq(4.0))
            .return_const(CharSample::Draw('X'));
        converter
            .expect_color()
            .with(eq(7.0))
            .return_const(Color::Red);
        converter
            .expect_color()
            .with(eq(8.0))
            .return_const(Color::Blue);

        let seq = &mut Sequence::new();

        // Constructor
        printer
            .expect_hide_cursor()
            .once()
            .returning(|| Ok(()))
            .in_sequence(seq);

        // Rendering
        printer
            .expect_move_to()
            .once()
            .with(eq(1), eq(0))
            .returning(|_, _| Ok(()))
            .in_sequence(seq);
        printer
            .expect_print()
            .once()
            .with(eq(' '))
            .returning(|_| Ok(()))
            .in_sequence(seq);
        printer
            .expect_move_to()
            .once()
            .with(eq(0), eq(1))
            .returning(|_, _| Ok(()))
            .in_sequence(seq);
        printer
            .expect_set_foreground()
            .once()
            .with(eq(Color::Red))
            .returning(|_| Ok(()))
            .in_sequence(seq);
        printer
            .expect_print()
            .once()
            .with(eq('A'))
            .returning(|_| Ok(()))
            .in_sequence(seq);
        printer
            .expect_move_to()
            .once()
            .with(eq(1), eq(1))
            .returning(|_, _| Ok(()))
            .in_sequence(seq);
        printer
            .expect_set_foreground()
            .once()
            .with(eq(Color::Blue))
            .returning(|_| Ok(()))
            .in_sequence(seq);
        printer
            .expect_print()
            .once()
            .with(eq('X'))
            .returning(|_| Ok(()))
            .in_sequence(seq);
        printer
            .expect_flush()
            .once()
            .returning(|| Ok(()))
            .in_sequence(seq);

        // Drop
        printer
            .expect_move_to()
            .once()
            .returning(|_, _| Ok(()))
            .in_sequence(seq);
        printer
            .expect_set_foreground()
            .with(eq(Color::Reset))
            .once()
            .returning(|_| Ok(()))
            .in_sequence(seq);
        printer
            .expect_show_cursor()
            .once()
            .returning(|| Ok(()))
            .in_sequence(seq);
        printer
            .expect_clear()
            .once()
            .returning(|| Ok(()))
            .in_sequence(seq);
        printer
            .expect_flush()
            .once()
            .returning(|| Ok(()))
            .in_sequence(seq);

        let mut renderer = RendererImpl::new(sampler, converter, printer).unwrap();

        renderer.render(0.0).unwrap();
    }

    #[test]
    fn render() {
        let mut sampler = MockSamplerFactory::new();
        let mut converter = MockConverter::new();
        let mut printer = MockPrinter::new();

        sampler.expect_create().returning(|_| {
            let mut sampler = MockSampler::new();
            sampler.expect_char().return_const(0.0);
            sampler
        });
        converter.expect_char().return_const(CharSample::Keep);
        printer.expect_size().returning(|| Ok((3, 2)));
        printer.expect_flush().returning(|| Ok(()));

        // Constructor
        printer.expect_hide_cursor().returning(|| Ok(()));

        // Drop
        printer.expect_move_to().returning(|_, _| Ok(()));
        printer.expect_set_foreground().returning(|_| Ok(()));
        printer.expect_show_cursor().returning(|| Ok(()));
        printer.expect_clear().returning(|| Ok(()));
        printer.expect_flush().returning(|| Ok(()));

        let mut renderer = RendererImpl::new(sampler, converter, printer).unwrap();

        renderer.render(0.8).unwrap();
    }

    #[test]
    fn end() {
        let factory = MockSamplerFactory::new();
        let converter = MockConverter::new();
        let mut printer = MockPrinter::new();

        let seq = &mut Sequence::new();

        // Constructor
        printer.expect_hide_cursor().returning(|| Ok(()));

        // Drop
        printer
            .expect_move_to()
            .with(eq(0), eq(0))
            .once()
            .returning(|_, _| Ok(()))
            .in_sequence(seq);
        printer
            .expect_set_foreground()
            .with(eq(Color::Reset))
            .once()
            .returning(|_| Ok(()))
            .in_sequence(seq);
        printer
            .expect_show_cursor()
            .once()
            .returning(|| Ok(()))
            .in_sequence(seq);
        printer
            .expect_clear()
            .once()
            .returning(|| Ok(()))
            .in_sequence(seq);
        printer
            .expect_flush()
            .once()
            .returning(|| Ok(()))
            .in_sequence(seq);

        drop(RendererImpl::new(factory, converter, printer));
    }
}

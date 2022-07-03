use anyhow::Error;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::{ExecutableCommand, QueueableCommand};
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use std::io::Write;
use crate::array::Array2D;

#[cfg(test)]
use mockall::automock;

/// A surface to draw characters on.
#[cfg_attr(test, automock)]
pub trait Surface {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    /// Overwrite the character on screen with this value.
    fn draw(&mut self, x: usize, y: usize, char: char, color: Color);

    /// Clear the character on screen.
    fn clear(&mut self, x: usize, y: usize);

    /// Present the finished frame.
    fn present(&mut self) -> Result<(), Error>;
}

/// Renders the frames into a [Write] struct.
pub struct WriteSurface<T: Write> {
    out: T,
    array: Array2D<Cell>,
}

#[derive(Copy, Clone)]
enum Cell {
    Keep,
    Draw { char: char, color: Color },
}

impl Default for Cell {
    fn default() -> Self { Cell::Keep }
}

impl<T: Write> WriteSurface<T> {
    pub fn new(mut out: T, width: usize, height: usize) -> Result<Self, Error> {
        out.queue(Hide)?;

        Ok(Self {
            out,
            array: Array2D::new(width, height)
        })
    }

    fn try_drop(&mut self) -> Result<(), Error> {
        self.out.queue(Show)?;
        self.out.execute(Clear(ClearType::Purge))?;
        Ok(())
    }
}

impl<T: Write> Surface for WriteSurface<T> {
    fn width(&self) -> usize {
        self.array.width()
    }

    fn height(&self) -> usize {
        self.array.height()
    }

    fn draw(&mut self, x: usize, y: usize, char: char, color: Color) {
        self.array[(x, y)] = Cell::Draw { char, color };
    }

    fn clear(&mut self, x: usize, y: usize) {
        self.array[(x, y)] = Cell::Draw { char: ' ', color: Color::Reset };
    }

    fn present(&mut self) -> Result<(), Error> {
        let mut needs_move;
        let mut last_color = None;

        for y in 0..self.array.height() {
            needs_move = true;

            for x in 0..self.array.width() {
                match self.array[(x, y)] {
                    Cell::Keep => {
                        needs_move = true;
                    }
                    Cell::Draw { char, color } => {
                        if needs_move {
                            needs_move = false;
                            self.out.queue(MoveTo(x as u16, y as u16))?;
                        }
                        if last_color.is_none() || last_color.unwrap() != color {
                            last_color = Some(color);
                            self.out.queue(SetForegroundColor(color))?;
                        }
                        self.out.queue(Print(char))?;
                    }
                }
            }
        }
        self.out.flush()?;
        Ok(())
    }
}

impl<T: Write> Drop for WriteSurface<T> {
    fn drop(&mut self) {
        if let Err(e) = self.try_drop() {
            println!("{}", e);
        }
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::*;

    #[derive(PartialEq, Debug)]
    struct Data {
        flushed: Vec<Vec<u8>>,
        buffer: Vec<u8>
    }

    struct MockWrite {
        data: Rc<RefCell<Data>>
    }

    impl Data {
        fn new() -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Self {
                flushed: Vec::new(),
                buffer: Vec::new()
            }))
        }
    }

    impl MockWrite {
        fn new(data: Rc<RefCell<Data>>) -> Box<dyn Write> {
            Box::new(Self { data })
        }
    }

    impl Write for MockWrite {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.borrow_mut().buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            let data = self.data.borrow_mut().buffer.drain(..).collect();

            self.data.borrow_mut().flushed.push(data);
            Ok(())
        }
    }

    #[test]
    fn width() {
        let data = Data::new();
        let mock = MockWrite::new(data);
        let surface = WriteSurface::new(mock, 10, 2).unwrap();

        assert_eq!(10, surface.width());
    }

    #[test]
    fn height() {
        let data = Data::new();
        let mock = MockWrite::new(data);
        let surface = WriteSurface::new(mock, 5, 8).unwrap();

        assert_eq!(8, surface.height());
    }

    #[test]
    fn present() {
        // Execute
        let data = Data::new();
        let mock = MockWrite::new(data.clone());
        let mut surface = WriteSurface::new(mock, 3, 2).unwrap();

        surface.draw(0, 0, 'A', Color::Green);
        surface.draw(1, 0, 'x', Color::Green);
        surface.clear(1, 1);
        surface.present().unwrap();

        drop(surface);

        // Recreate expectation
        let expected = Data::new();
        let mut stream = MockWrite::new(expected.clone());

        stream.queue(Hide).unwrap();
        stream.queue(MoveTo(0, 0)).unwrap();
        stream.queue(SetForegroundColor(Color::Green)).unwrap();
        stream.queue(Print('A')).unwrap();
        stream.queue(Print('x')).unwrap();
        stream.queue(MoveTo(1, 1)).unwrap();
        stream.queue(SetForegroundColor(Color::Reset)).unwrap();
        stream.queue(Print(' ')).unwrap();
        stream.flush().unwrap();
        stream.queue(Show).unwrap();
        stream.queue(Clear(ClearType::Purge)).unwrap();
        stream.flush().unwrap();

        // Compare
        assert_eq!(expected, data);
    }
}
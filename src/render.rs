use anyhow::Error;
use crossterm::cursor::MoveTo;
use crossterm::{ExecutableCommand, QueueableCommand};
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use std::io::Write;
use crate::array::Array2D;

pub trait Renderer {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn draw(&mut self, x: usize, y: usize, char: char, color: Color);
    fn clear(&mut self, x: usize, y: usize);
    fn render(&mut self) -> Result<(), Error>;
    fn purge(&mut self) -> Result<(), Error>;
}

pub struct WriteRenderer {
    out: Box<dyn Write>,
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

impl WriteRenderer {
    pub fn new(out: Box<dyn Write>, width: usize, height: usize) -> Self {
        Self {
            out,
            array: Array2D::new(width, height)
        }
    }
}

impl Renderer for WriteRenderer {
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

    fn render(&mut self) -> Result<(), Error> {
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
        self.out.queue(MoveTo(0, 0))?;
        self.out.flush()?;
        Ok(())
    }

    fn purge(&mut self) -> Result<(), Error> {
        self.out.execute(Clear(ClearType::Purge))?;
        Ok(())
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
        let renderer = WriteRenderer::new(mock, 10, 2);

        assert_eq!(10, renderer.width());
    }

    #[test]
    fn height() {
        let data = Data::new();
        let mock = MockWrite::new(data);
        let renderer = WriteRenderer::new(mock, 5, 8);

        assert_eq!(8, renderer.height());
    }

    #[test]
    fn purge() {
        // Execute
        let data = Data::new();
        let mock = MockWrite::new(data.clone());
        let mut renderer = WriteRenderer::new(mock, 4, 4);

        renderer.purge().unwrap();

        // Recreate expectation
        let expected = Data::new();
        let mut stream = MockWrite::new(expected.clone());

        stream.execute(Clear(ClearType::Purge)).unwrap();

        // Compare
        assert_eq!(expected, data);
    }

    #[test]
    fn render() {
        // Execute
        let data = Data::new();
        let mock = MockWrite::new(data.clone());
        let mut renderer = WriteRenderer::new(mock, 3, 2);

        renderer.draw(0, 0, 'A', Color::Green);
        renderer.draw(1, 0, 'x', Color::Green);
        renderer.clear(1, 1);
        renderer.render().unwrap();

        // Recreate expectation
        let expected = Data::new();
        let mut stream = MockWrite::new(expected.clone());

        stream.queue(MoveTo(0, 0)).unwrap();
        stream.queue(SetForegroundColor(Color::Green)).unwrap();
        stream.queue(Print('A')).unwrap();
        stream.queue(Print('x')).unwrap();
        stream.queue(MoveTo(1, 1)).unwrap();
        stream.queue(SetForegroundColor(Color::Reset)).unwrap();
        stream.queue(Print(' ')).unwrap();
        stream.queue(MoveTo(0, 0)).unwrap();
        stream.flush().unwrap();

        // Compare
        assert_eq!(expected, data);
    }
}
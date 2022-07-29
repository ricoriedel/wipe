use crate::error::Error;
use crate::term::Terminal;
use crossterm::cursor::*;
use crossterm::style::*;
use crossterm::terminal::*;

pub trait Printer {
    fn show_cursor(&mut self) -> Result<(), Error>;
    fn hide_cursor(&mut self) -> Result<(), Error>;
    fn print(&mut self, char: char) -> Result<(), Error>;
    fn move_to(&mut self, x: u16, y: u16) -> Result<(), Error>;
    fn size(&self) -> Result<(u16, u16), Error>;
    fn set_foreground(&mut self, color: Color) -> Result<(), Error>;
    fn clear(&mut self) -> Result<(), Error>;
    fn flush(&mut self) -> Result<(), Error>;
}

pub struct PrinterImpl<T> {
    term: T,
    cursor: Option<bool>,
    foreground: Option<Color>,
}

impl<T> PrinterImpl<T> {
    pub fn new(term: T) -> Self {
        Self {
            term,
            cursor: None,
            foreground: None,
        }
    }
}

impl<T: Terminal> Printer for PrinterImpl<T> {
    fn show_cursor(&mut self) -> Result<(), Error> {
        if self.cursor != Some(true) {
            self.cursor = Some(true);
            self.term.queue(Show)?;
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), Error> {
        if self.cursor != Some(false) {
            self.cursor = Some(false);
            self.term.queue(Hide)?;
        }
        Ok(())
    }

    fn print(&mut self, char: char) -> Result<(), Error> {
        self.term.queue(Print(char))?;
        Ok(())
    }

    fn move_to(&mut self, x: u16, y: u16) -> Result<(), Error> {
        if (x, y) != self.term.position()? {
            self.term.queue(MoveTo(x, y))?;
        }
        Ok(())
    }

    fn size(&self) -> Result<(u16, u16), Error> {
        self.term.size()
    }

    fn set_foreground(&mut self, color: Color) -> Result<(), Error> {
        if self.foreground == Some(color) {
            self.foreground = Some(color);
            self.term.queue(SetForegroundColor(color))?;
        }
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Error> {
        self.term.queue(Clear(ClearType::Purge))?;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.term.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::term::MockTerminal;
    use mockall::predicate::eq;

    #[test]
    fn show_cursor() {
        let mut mock = MockTerminal::new();
        mock.expect_queue()
            .with(eq(Show))
            .once()
            .returning(|_| Ok(()));

        PrinterImpl::new(mock).show_cursor().unwrap();
    }

    #[test]
    fn show_cursor_twice_queues_once() {
        let mut mock = MockTerminal::new();
        mock.expect_queue()
            .with(eq(Show))
            .once()
            .returning(|_| Ok(()));

        let mut printer = PrinterImpl::new(mock);

        printer.show_cursor().unwrap();
        printer.show_cursor().unwrap();
    }

    #[test]
    fn show_cursor_after_hiding_queues_show() {
        let mut mock = MockTerminal::new();
        mock.expect_queue()
            .with(eq(Show))
            .once()
            .returning(|_| Ok(()));
        mock.expect_queue().with(eq(Hide)).returning(|_| Ok(()));

        let mut printer = PrinterImpl::new(mock);

        printer.hide_cursor().unwrap();
        printer.show_cursor().unwrap();
    }

    #[test]
    fn hide_cursor() {
        let mut mock = MockTerminal::new();
        mock.expect_queue()
            .with(eq(Hide))
            .once()
            .returning(|_| Ok(()));

        PrinterImpl::new(mock).hide_cursor().unwrap();
    }

    #[test]
    fn hide_cursor_twice_queues_once() {
        let mut mock = MockTerminal::new();
        mock.expect_queue()
            .with(eq(Hide))
            .once()
            .returning(|_| Ok(()));

        let mut printer = PrinterImpl::new(mock);

        printer.hide_cursor().unwrap();
        printer.hide_cursor().unwrap();
    }

    #[test]
    fn hide_cursor_after_showing_queues_hide() {
        let mut mock = MockTerminal::new();
        mock.expect_queue()
            .with(eq(Show))
            .once()
            .returning(|_| Ok(()));
        mock.expect_queue().with(eq(Hide)).returning(|_| Ok(()));

        let mut printer = PrinterImpl::new(mock);

        printer.show_cursor().unwrap();
        printer.hide_cursor().unwrap();
    }

    #[test]
    fn print() {
        let mut mock = MockTerminal::new();
        mock.expect_queue()
            .with(eq(Print('R')))
            .once()
            .returning(|_| Ok(()));

        PrinterImpl::new(mock).print('R').unwrap();
    }

    #[test]
    fn move_to_different_position_queues() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((7, 2)));
        mock.expect_queue()
            .with(eq(MoveTo(5, 4)))
            .once()
            .returning(|_| Ok(()));

        PrinterImpl::new(mock).move_to(5, 4).unwrap();
    }

    #[test]
    fn move_to_same_position_does_not_queue() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((3, 13)));

        PrinterImpl::new(mock).move_to(3, 13).unwrap();
    }

    #[test]
    fn size() {
        let mut mock = MockTerminal::new();
        mock.expect_size().returning(|| Ok((14, 76)));

        assert_eq!((14, 76), PrinterImpl::new(mock).size().unwrap());
    }

    #[test]
    fn clear() {
        let mut mock = MockTerminal::new();
        mock.expect_queue()
            .with(eq(Clear(ClearType::Purge)))
            .once()
            .returning(|_| Ok(()));

        PrinterImpl::new(mock).clear().unwrap();
    }

    #[test]
    fn flush() {
        let mut mock = MockTerminal::new();
        mock.expect_flush().once().returning(|| Ok(()));

        PrinterImpl::new(mock).flush().unwrap();
    }
}

use crate::Error;
use crate::Terminal;
use crossterm::cursor::*;
use crossterm::style::*;
use crossterm::terminal::*;

#[cfg_attr(test, mockall::automock)]
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
    position: (u16, u16),
    cursor: Option<bool>,
    foreground: Option<Color>,
}

impl<T: Terminal> PrinterImpl<T> {
    pub fn new(term: T) -> Result<Self, Error> {
        let position = term.position()?;

        Ok(Self {
            term,
            position,
            cursor: None,
            foreground: None,
        })
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
        if char < '\u{20}' || char == '\u{7F}' {
            return Err("Special chars can't be printed.".into());
        }
        self.position.0 += 1;
        self.term.queue(Print(char))?;
        Ok(())
    }

    fn move_to(&mut self, x: u16, y: u16) -> Result<(), Error> {
        if self.position != (x, y) {
            self.position = (x, y);
            self.term.queue(MoveTo(x, y))?;
        }
        Ok(())
    }

    fn size(&self) -> Result<(u16, u16), Error> {
        self.term.size()
    }

    fn set_foreground(&mut self, color: Color) -> Result<(), Error> {
        if self.foreground != Some(color) {
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
    use crate::MockTerminal;
    use mockall::predicate::eq;

    #[test]
    fn show_cursor() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((0, 0)));
        mock.expect_queue()
            .with(eq(Show))
            .once()
            .returning(|_| Ok(()));

        PrinterImpl::new(mock).unwrap().show_cursor().unwrap();
    }

    #[test]
    fn show_cursor_twice_queues_once() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((0, 0)));
        mock.expect_queue()
            .with(eq(Show))
            .once()
            .returning(|_| Ok(()));

        let mut printer = PrinterImpl::new(mock).unwrap();

        printer.show_cursor().unwrap();
        printer.show_cursor().unwrap();
    }

    #[test]
    fn show_cursor_after_hiding_queues_show() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((0, 0)));
        mock.expect_queue()
            .with(eq(Show))
            .once()
            .returning(|_| Ok(()));
        mock.expect_queue().with(eq(Hide)).returning(|_| Ok(()));

        let mut printer = PrinterImpl::new(mock).unwrap();

        printer.hide_cursor().unwrap();
        printer.show_cursor().unwrap();
    }

    #[test]
    fn hide_cursor() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((0, 0)));
        mock.expect_queue()
            .with(eq(Hide))
            .once()
            .returning(|_| Ok(()));

        PrinterImpl::new(mock).unwrap().hide_cursor().unwrap();
    }

    #[test]
    fn hide_cursor_twice_queues_once() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((0, 0)));
        mock.expect_queue()
            .with(eq(Hide))
            .once()
            .returning(|_| Ok(()));

        let mut printer = PrinterImpl::new(mock).unwrap();

        printer.hide_cursor().unwrap();
        printer.hide_cursor().unwrap();
    }

    #[test]
    fn hide_cursor_after_showing_queues_hide() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((0, 0)));
        mock.expect_queue()
            .with(eq(Show))
            .once()
            .returning(|_| Ok(()));
        mock.expect_queue().with(eq(Hide)).returning(|_| Ok(()));

        let mut printer = PrinterImpl::new(mock).unwrap();

        printer.show_cursor().unwrap();
        printer.hide_cursor().unwrap();
    }

    #[test]
    fn set_foreground() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((0, 0)));
        mock.expect_queue()
            .with(eq(SetForegroundColor(Color::Blue)))
            .once()
            .returning(|_| Ok(()));

        PrinterImpl::new(mock)
            .unwrap()
            .set_foreground(Color::Blue)
            .unwrap();
    }

    #[test]
    fn set_foreground_twice_queues_once() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((0, 0)));
        mock.expect_queue()
            .once()
            .returning(|_: SetForegroundColor| Ok(()));

        let mut printer = PrinterImpl::new(mock).unwrap();

        printer.set_foreground(Color::Red).unwrap();
        printer.set_foreground(Color::Red).unwrap();
    }

    #[test]
    fn set_foreground_different_color_queues() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((0, 0)));
        mock.expect_queue()
            .times(3)
            .returning(|_: SetForegroundColor| Ok(()));

        let mut printer = PrinterImpl::new(mock).unwrap();

        printer.set_foreground(Color::Red).unwrap();
        printer.set_foreground(Color::Blue).unwrap();
        printer.set_foreground(Color::Red).unwrap();
    }

    #[test]
    fn print() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((0, 0)));
        mock.expect_queue()
            .with(eq(Print('R')))
            .once()
            .returning(|_| Ok(()));

        PrinterImpl::new(mock).unwrap().print('R').unwrap();
    }

    #[test]
    fn print_moves_cursor() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((2, 4)));
        mock.expect_queue()
            .times(3)
            .returning(|_: Print<char>| Ok(()));

        let mut printer = PrinterImpl::new(mock).unwrap();

        printer.print('A').unwrap();
        printer.print('B').unwrap();
        printer.print('C').unwrap();
        printer.move_to(5, 4).unwrap();
    }

    #[test]
    fn print_special_char_fails() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((2, 4)));

        let mut printer = PrinterImpl::new(mock).unwrap();

        assert!(printer.print('\u{0}').is_err());
        assert!(printer.print('\u{1F}').is_err());
        assert!(printer.print('\u{7F}').is_err());
    }

    #[test]
    fn move_to_different_position_queues() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((7, 2)));
        mock.expect_queue()
            .with(eq(MoveTo(5, 4)))
            .once()
            .returning(|_| Ok(()));

        PrinterImpl::new(mock).unwrap().move_to(5, 4).unwrap();
    }

    #[test]
    fn move_to_same_position_does_not_queue() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((3, 13)));

        PrinterImpl::new(mock).unwrap().move_to(3, 13).unwrap();
    }

    #[test]
    fn size() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((0, 0)));
        mock.expect_size().returning(|| Ok((14, 76)));

        assert_eq!((14, 76), PrinterImpl::new(mock).unwrap().size().unwrap());
    }

    #[test]
    fn clear() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((0, 0)));
        mock.expect_queue()
            .with(eq(Clear(ClearType::Purge)))
            .once()
            .returning(|_| Ok(()));

        PrinterImpl::new(mock).unwrap().clear().unwrap();
    }

    #[test]
    fn flush() {
        let mut mock = MockTerminal::new();
        mock.expect_position().returning(|| Ok((0, 0)));
        mock.expect_flush().once().returning(|| Ok(()));

        PrinterImpl::new(mock).unwrap().flush().unwrap();
    }
}

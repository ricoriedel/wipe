use crate::error::Error;
use crossterm::{Command, QueueableCommand};
use std::io::Write;

#[cfg_attr(test, mockall::automock)]
pub trait Terminal {
    fn queue<T: 'static + Command>(&mut self, cmd: T) -> Result<(), Error>;
    fn flush(&mut self) -> Result<(), Error>;
    fn size(&self) -> Result<(u16, u16), Error>;
    fn position(&self) -> Result<(u16, u16), Error>;
}

pub struct TerminalImpl<T> {
    out: T,
}

impl<T> TerminalImpl<T> {
    pub fn new(out: T) -> Self {
        Self { out }
    }
}

impl<T: Write> Terminal for TerminalImpl<T> {
    fn queue<TCmd: Command>(&mut self, cmd: TCmd) -> Result<(), Error> {
        self.out.queue(cmd)?;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.out.flush()?;
        Ok(())
    }

    fn size(&self) -> Result<(u16, u16), Error> {
        Ok(crossterm::terminal::size()?)
    }

    fn position(&self) -> Result<(u16, u16), Error> {
        Ok(crossterm::cursor::position()?)
    }
}

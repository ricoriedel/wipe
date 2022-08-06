use crate::Error;
use crossterm::{Command, QueueableCommand};
use std::io::Write;

/// A stub for OS calls and crossterm functions.
#[cfg_attr(test, mockall::automock)]
pub trait Terminal {
    /// Queues a command for execution.
    fn queue<T: 'static + Command>(&mut self, cmd: T) -> Result<(), Error>;
    /// Flushes all queued commands.
    fn flush(&mut self) -> Result<(), Error>;
    /// Returns the current size of the terminal.
    fn size(&self) -> Result<(u16, u16), Error>;
    /// Returns the current cursor position.
    fn position(&self) -> Result<(u16, u16), Error>;
}

/// The implementation of [Terminal].
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

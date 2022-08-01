pub mod convert;
pub mod pattern;

mod error;
mod printer;
mod renderer;
mod term;
mod timer;
mod vec;

pub use error::*;
pub use printer::*;
pub use renderer::*;
pub use term::*;
pub use timer::*;
pub use vec::*;

fn main() -> Result<(), Error> {
    Ok(())
}

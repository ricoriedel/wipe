pub mod convert;
pub mod error;
pub mod pattern;
pub mod printer;
mod renderer;
pub mod term;
mod vec;

fn main() -> Result<(), error::Error> {
    Ok(())
}

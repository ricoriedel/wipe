//! Contains transformations to apply on top of patterns.

mod invert;
mod segment;
mod shift;
mod slice;
mod swap;

pub use invert::*;
pub use segment::*;
pub use shift::*;
pub use slice::*;
pub use swap::*;

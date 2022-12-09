pub use crate::shape::{Shape, ShapeKind};

pub const ROUND_SIZE: usize = 2;

pub type Result<T> = std::result::Result<T, &'static str>;
pub type Round = [Shape; ROUND_SIZE];

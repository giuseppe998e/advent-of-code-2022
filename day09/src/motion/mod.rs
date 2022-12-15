use crate::prelude::*;

mod direction;
pub use direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Motion {
    direction: Direction,
    steps: u8,
}

impl Motion {
    pub fn new(direction: &str, steps: u8) -> Result<Self> {
        let direction = Direction::new(direction)?;
        Ok(Self { direction, steps })
    }
}

impl TryFrom<&str> for Motion {
    type Error = &'static str;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();

        let direction = parts.next().ok_or("The input line is malformed!")?;
        let steps = parts
            .next()
            .and_then(|v| v.parse::<u8>().ok())
            .ok_or("The input line is malformed!")?;

        Self::new(direction, steps)
    }
}

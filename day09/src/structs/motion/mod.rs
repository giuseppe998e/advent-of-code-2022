use std::ops::{Add, Sub};

mod direction;
pub use direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Motion {
    pub direction: Direction,
    pub steps: u8,
}

impl Motion {
    pub fn new(direction: Direction, steps: u8) -> Self {
        Self { direction, steps }
    }
}

impl TryFrom<&str> for Motion {
    type Error = &'static str;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();

        let direction = parts
            .next()
            .ok_or("The input line is malformed!")
            .and_then(Direction::try_from)?;
        let steps = parts
            .next()
            .and_then(|v| v.parse::<u8>().ok())
            .ok_or("The input line is malformed!")?;

        Ok(Self::new(direction, steps))
    }
}

impl Add<u8> for Motion {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self::new(self.direction, self.steps + rhs)
    }
}

impl Sub<u8> for Motion {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        Self::new(self.direction, self.steps - rhs)
    }
}


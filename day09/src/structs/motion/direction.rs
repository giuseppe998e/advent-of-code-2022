use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn new(direction: &str) -> Result<Self> {
        match direction {
            "U" => Ok(Self::Up),
            "L" => Ok(Self::Left),
            "D" => Ok(Self::Down),
            "R" => Ok(Self::Right),
            _ => Err("The given direction is unknown!"),
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = &'static str;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Self::new(value)
    }
}


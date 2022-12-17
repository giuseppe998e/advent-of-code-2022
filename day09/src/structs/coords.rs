use crate::prelude::{Direction, Motion};
use std::{cmp::Ordering, ops::Add};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coords {
    pub x: u8,
    pub y: u8,
}

impl Coords {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Coords) -> (u8, u8) {
        (self.x.abs_diff(other.x), self.y.abs_diff(other.y))
    }

    pub fn to(&self, other: &Coords) -> [Option<Motion>; 2] {
        let (distance_x, distance_y) = self.distance(other);

        let horizontal_motion = match self.x.cmp(&other.x) {
            Ordering::Greater => Some(Motion::new(Direction::Left, distance_x)),
            Ordering::Less => Some(Motion::new(Direction::Right, distance_x)),
            Ordering::Equal => None,
        };

        let vertical_motion = match self.y.cmp(&other.y) {
            Ordering::Greater => Some(Motion::new(Direction::Down, distance_y)),
            Ordering::Less => Some(Motion::new(Direction::Up, distance_y)),
            Ordering::Equal => None,
        };

        [horizontal_motion, vertical_motion]
    }
}

impl Add<Motion> for Coords {
    type Output = Self;

    fn add(mut self, motion: Motion) -> Self::Output {
        match motion.direction {
            // Move ahead
            Direction::Right => self.x += motion.steps,
            Direction::Up => self.y += motion.steps,
            // Move backward
            Direction::Left => self.x -= motion.steps,
            Direction::Down => self.y -= motion.steps,
        };

        self
    }
}

//impl Sub<Motion> for Coords {
//    type Output = Self;
//
//    fn sub(self, motion: Motion) -> Self::Output {
//        self.add(motion)
//    }
//}


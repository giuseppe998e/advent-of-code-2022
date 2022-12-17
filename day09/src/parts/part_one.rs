use crate::prelude::*;

pub fn part_one(motions: &[Motion]) -> usize {
    let head = Coords::new(5, 3);
    let tail = Coords::default();

    println!("{:?}", head.to(&tail));

    0
}


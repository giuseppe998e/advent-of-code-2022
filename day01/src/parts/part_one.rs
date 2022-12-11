use crate::prelude::*;

pub fn part_one(elves: &[u32]) -> Result<u32> {
    let value = elves
        .iter()
        .max()
        .ok_or("Cannot find the value of maximum calories!")?;

    Ok(*value)
}

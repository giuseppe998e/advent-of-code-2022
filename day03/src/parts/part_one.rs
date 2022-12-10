use std::collections::HashSet;

use crate::prelude::*;

fn calc_single_rucksack(rucksack: &Rucksack) -> Result<u32> {
    let (first, second) = rucksack;
    let (first, second) = (
        first.chars().collect::<HashSet<char>>(),
        second.chars().collect::<HashSet<char>>(),
    );

    first
        .iter()
        .find_map(|b| second.iter().find(|&a| a == b))
        .map(char::priority)
        .ok_or("There should be exactly one duplicate!")
}

pub fn part_one(rucksacks: &[Rucksack]) -> Result<u32> {
    rucksacks.iter().map(calc_single_rucksack).sum()
}

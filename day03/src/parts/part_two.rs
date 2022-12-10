use std::collections::HashSet;

use crate::prelude::*;

pub fn calc_single_group(rucksacks: &[Rucksack]) -> Result<u32> {
    let [first, second, third] = rucksacks else {
        return Err("Groups of rucksacks must be mandatory of 3!");
    };

    let (first, second, third) = (
        first.0
            .chars()
            .chain(first.1.chars())
            .collect::<HashSet<char>>(),
        second.0
            .chars()
            .chain(second.1.chars())
            .collect::<HashSet<char>>(),
        third.0
            .chars()
            .chain(third.1.chars())
            .collect::<HashSet<char>>(),
    );

    first
        .iter()
        .find_map(|f| {
            second
                .iter()
                .find_map(|s| third.iter().find(|&t| f == s && s == t))
        })
        .map(char::priority)
        .ok_or("There should be exactly one duplicate!")
}

pub fn part_two(rucksacks: &[Rucksack]) -> Result<u32> {
    rucksacks.chunks_exact(3).map(calc_single_group).sum()
}

use crate::prelude::*;

pub fn part_one(assignments: &[Assignment]) -> usize {
    assignments
        .iter()
        .filter(|(range_a, range_b)| {
            (range_a.0 >= range_b.0 && range_a.1 <= range_b.1)
                || (range_b.0 >= range_a.0 && range_b.1 <= range_a.1)
        })
        .count()
}

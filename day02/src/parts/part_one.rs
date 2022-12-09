use std::cmp::Ordering;

use crate::prelude::*;

fn calc_round_score(round: &Round) -> u32 {
    let (me, rival) = if round[1].is_rival {
        (round[0], round[1])
    } else {
        (round[1], round[0])
    };

    let score = match me.cmp(&rival) {
        // Win
        Ordering::Greater => 6,
        // Draw
        Ordering::Equal => 3,
        // Lose
        Ordering::Less => 0,
    };

    me.value() + score
}

pub fn part_one(rounds: &[Round]) -> u32 {
    rounds.iter().map(calc_round_score).sum()
}

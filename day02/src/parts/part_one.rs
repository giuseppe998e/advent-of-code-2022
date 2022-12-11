use std::cmp::Ordering;

use crate::prelude::*;

fn calc_round_score(round: &Round) -> u32 {
    let rival_first = round[0].is_rival;
    let (me, rival) = (round[rival_first as usize], round[!rival_first as usize]);

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

use crate::prelude::*;

fn calc_round_score(round: &Round) -> u32 {
    let (rival, end) = (round[0], round[1]);

    match end.kind {
        // Z == Scissors == Win
        ShapeKind::Scissors => 6 + rival.winning_shape().value(),
        // Y == Paper == Draw
        ShapeKind::Paper => 3 + rival.value(),
        // X == Rock == Lose
        ShapeKind::Rock => rival.losing_shape().value(),
    }
}

pub fn part_two(rounds: &[Round]) -> u32 {
    rounds.iter().map(calc_round_score).sum()
}

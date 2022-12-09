mod prelude;
mod shape;
use prelude::*;

mod parts;
use parts::part_one;

use crate::parts::part_two;

fn parse_rounds(input: &str) -> Result<Vec<Round>> {
    let shapes = input
        .split_whitespace()
        .map(Shape::new)
        .collect::<Vec<Shape>>();

    shapes
        .chunks_exact(ROUND_SIZE)
        .map(|chunk| {
            chunk
                .try_into()
                .map_err(|_| "The length of the round is not equal to 2!")
        })
        .collect()
}

fn main() -> Result<()> {
    // Parse rounds
    let input = include_str!("input.txt");
    let rounds = parse_rounds(input)?;

    // Part one
    let result = part_one(&rounds);
    println!("[Part one] Match score: {}", result);

    // Part two
    let result = part_two(&rounds);
    println!("[Part two] Match score: {}", result);

    Ok(())
}

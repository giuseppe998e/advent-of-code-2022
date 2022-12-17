mod structs;

mod prelude;
use prelude::*;

mod parts;
use parts::part_one;

fn parse_motions(input: &str) -> Result<Box<[Motion]>> {
    input
        .lines()
        .map(Motion::try_from)
        .collect::<Result<_>>()
        .map(Vec::into_boxed_slice)
}

fn main() -> Result<()> {
    // Parse motion list
    let input = include_str!("input.txt");
    let motions = parse_motions(input)?;

    // Part one
    let result = part_one(&motions);
    println!("[Part one] Positions visited at least once: {}", result);

    // Part two
    // TODO

    Ok(())
}


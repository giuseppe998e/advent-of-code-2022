mod motion;
mod prelude;
use prelude::*;

fn parse_motions(input: &str) -> Result<Vec<Motion>> {
    input.lines().map(Motion::try_from).collect()
}

fn main() -> Result<()> {
    // Parse motion list
    let input = include_str!("input.txt");
    let motions = parse_motions(input)?;

    // Part one
    // TODO

    // Part two
    // TODO

    Ok(())
}

mod prelude;
use prelude::*;

mod parts;
use parts::{part_one, part_two};

fn parse_assignments(input: &str) -> Result<Vec<Assignment>> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|pair| {
                    pair.split('-')
                        .map(|number| {
                            number
                                .parse::<u32>()
                                .map_err(|_| "The given input contains a non-number value!")
                        })
                        .collect::<Result<Vec<u32>>>()
                        .map(|vec| (vec[0], vec[1]))
                })
                .collect::<Result<Vec<Pair>>>()
                .map(|vec| (vec[0], vec[1]))
        })
        .collect()
}

fn main() -> Result<()> {
    // Parse assignments
    let input = include_str!("input.txt");
    let assignments = parse_assignments(input)?;

    // Part one
    let result = part_one(&assignments);
    println!("[Part one] Pairs that fully contains others: {:?}", result);

    // Part two
    let result = part_two(&assignments);
    println!("[Part two] Pairs that overlaps others: {:?}", result);

    Ok(())
}

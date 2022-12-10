mod char_trait;

mod prelude;
use prelude::*;

mod parts;
use parts::{part_one, part_two};

fn parse_rucksacks(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect()
}

fn main() -> Result<()> {
    // Parse rucksack
    let input = include_str!("input.txt");
    let rucksacks = parse_rucksacks(input);

    // Part one
    let result = part_one(&rucksacks)?;
    println!("[Part one] Rucksack priorities sum: {}", result);

    // Part two
    let result = part_two(&rucksacks)?;
    println!("[Part two] Groups of 3 priorities sum: {}", result);

    Ok(())
}

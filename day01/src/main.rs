mod prelude;
use prelude::*;

mod parts;
use parts::{part_one, part_two};

fn parse_elves(input: &str) -> Result<Vec<u32>> {
    let mut elves = vec![0u32];

    for line in input.lines() {
        if line.is_empty() {
            elves.push(0);
            continue;
        }

        let calories = line
            .parse::<u32>()
            .map_err(|_| "A non-number found in the input!")?;

        if let Some(entry) = elves.last_mut() {
            *entry += calories;
        }
    }

    Ok(elves)
}

fn main() -> Result<()> {
    // Parse elves calories
    let input = include_str!("input.txt");
    let mut elves = parse_elves(input)?;

    // Part one
    let result = part_one(&elves)?;
    println!("[Part one] Elf calories: {:?}", result);

    // Part two
    let result = part_two(&mut elves);
    println!("[Part two] First 3 Elves calories sum: {:?}", result);

    Ok(())
}

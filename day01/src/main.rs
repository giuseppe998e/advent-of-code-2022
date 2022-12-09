mod parts;
use crate::parts::{part_one, part_two};

fn parse_elves(input: &str, elves: &mut Vec<u32>) -> Result<(), &'static str> {
    for line in input.lines() {
        match line.is_empty() {
            true => elves.push(0),
            false => {
                let calories: u32 = line.parse().map_err(|_| "A non-number found in the input!")?;
                elves.last_mut().map(|val| *val += calories);
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), &'static str> {
    // Parse elves calories
    let input = include_str!("input.txt");
    let mut elves: Vec<u32> = vec![0];
    parse_elves(input, &mut elves)?;
    
    // Part one
    let (index, value) = part_one(&mut elves)?;
    println!("[Part one] Elve index: {:?}", index);
    println!("[Part one] Elve value: {:?}", value);

    // Part two
    let result = part_two(&mut elves);
    println!("[Part two] First 3 Elves total calories: {:?}", result);

    Ok(())
}

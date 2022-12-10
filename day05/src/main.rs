use std::str::{Chars, Lines};

mod vector_trait;
use vector_trait::AodVecTrait;

mod prelude;
use prelude::*;

mod parts;
use parts::{part_one, part_two};

fn parse_stacks_line(characters: &mut Chars, stacks: &mut Stacks) -> Result<()> {
    let mut stack_idx: usize = 0;
    let mut whitespaces: u8 = 0;

    while let Some(character) = characters.next() {
        match character {
            '[' => {
                let value = characters.next().ok_or("The input is malformed!")?;
                let stack = stacks.get_or_default_mut(stack_idx);
                stack.insert(0, value); // prepend
            }
            ']' => {
                characters.next();
                whitespaces += 1;
                stack_idx += 1;
            }
            _ if character.is_whitespace() => {
                whitespaces += 1;

                if whitespaces > 3 {
                    stacks.get_or_default_mut(stack_idx);
                    whitespaces = 0;
                    stack_idx += 1;
                }
            }
            _ => break,
        };
    }

    Ok(())
}

fn parse_stacks(input: &mut Lines) -> Result<Stacks> {
    let mut stacks: Stacks = Vec::with_capacity(9);

    for line in input {
        match line {
            _ if line.is_empty() => break,
            _ => {
                let mut characters = line.chars();
                parse_stacks_line(&mut characters, &mut stacks)?;
            }
        }
    }

    Ok(stacks)
}

fn parse_procedures(input: &mut Lines) -> Result<Procedures> {
    let mut procedures: Vec<Procedure> = Vec::with_capacity(8);

    for line in input {
        let mut words = line.split_whitespace();

        words.next();
        let quantity = words
            .next()
            .and_then(|word| word.parse::<u32>().ok())
            .ok_or("The procedure does not include a \"quantity\"!")?;

        words.next();
        let from = words
            .next()
            .and_then(|word| word.parse::<usize>().ok())
            .ok_or("The procedure does not include a \"from\" position!")?;

        words.next();
        let to = words
            .next()
            .and_then(|word| word.parse::<usize>().ok())
            .ok_or("The procedure does not include a \"to\" position!")?;

        procedures.push((quantity, from - 1, to - 1));
    }

    Ok(procedures.into_boxed_slice())
}

fn parse_supplies(input: &str) -> Result<(Stacks, Procedures)> {
    let mut input = input.lines();

    let stacks = parse_stacks(&mut input)?;
    let procedures = parse_procedures(&mut input)?;

    Ok((stacks, procedures))
}

fn main() -> Result<()> {
    // Parse stacks & procedures
    let input = include_str!("input.txt");
    let (mut stacks, procedures) = parse_supplies(input)?;

    // Part one
    let result = part_one(&mut stacks.clone(), &procedures)?;
    println!("[Part one] Top of stacks: {}", result);

    // Part two
    let result = part_two(&mut stacks, &procedures)?;
    println!("[Part two] Top of stacks: {}", result);

    Ok(())
}

mod util;

mod parts;
use parts::{part_one, part_two};

type Result<T> = std::result::Result<T, &'static str>;

fn parse_forest(input: &str) -> Result<(Box<[u32]>, usize)> {
    let mut columns = 0usize;

    input
        .lines()
        .flat_map(|line| {
            columns += 1; // In the input columns == rows

            line.chars()
                .map(|digit| digit.to_digit(10).ok_or("The given input is malformed!"))
        })
        .collect::<Result<_>>()
        .map(Vec::into_boxed_slice)
        .map(|forest| (forest, columns))
}

fn main() -> Result<()> {
    // Parse forest
    let input = include_str!("input.txt");
    let (forest, columns) = parse_forest(input)?;

    // Part one
    let result = part_one(&forest, columns);
    println!("[Part one] Sum of visible trees: {}", result);

    // Part two
    let result = part_two(&forest, columns);
    println!("[Part two] Highest scenic score: {}", result);

    Ok(())
}

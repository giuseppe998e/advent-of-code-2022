mod tree;
use tree::parse_tree;

mod prelude;
use prelude::*;

mod parts;
use parts::{part_one, part_two};

fn main() -> Result<()> {
    // Parse filesystem tree
    let input = include_str!("input.txt");
    let tree = parse_tree(input)?;

    // Part one
    let result = part_one(&tree)?;
    println!(
        "[Part one] Sum of dirs with '<=100_000' in size: {}",
        result
    );

    // Part two
    let result = part_two(&tree)?;
    println!("[Part two] Smallest deletable dir: {}", result);

    Ok(())
}

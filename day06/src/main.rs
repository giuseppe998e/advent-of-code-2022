type Result<T> = std::result::Result<T, &'static str>;

fn part_one_and_two(datastream: &[char], buffer: &mut [char], buffer_size: usize) -> Result<usize> {
    let loops = datastream.len() - buffer_size - 1;

    for start in 0..loops {
        let end = start + buffer_size;

        buffer.clone_from_slice(&datastream[start..end]);
        buffer.sort_unstable();

        if buffer.windows(2).all(|arr| arr[0] != arr[1]) {
            return Ok(end);
        }
    }

    Err("There is no starting point in the given string!")
}

fn main() -> Result<()> {
    // Parse datastream
    let input = include_str!("input.txt");
    let datastream = input.chars().collect::<Vec<char>>();

    // Part one
    let mut buffer = ['\0'; 4];
    let result = part_one_and_two(&datastream, &mut buffer, 4)?;
    println!("[Part one] Chars readed before first SoP: {}", result);

    // Part two
    let mut buffer = ['\0'; 14];
    let result = part_one_and_two(&datastream, &mut buffer, 14)?;
    println!("[Part two] Chars readed before first SoM: {}", result);

    Ok(())
}

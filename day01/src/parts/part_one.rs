pub fn part_one(elves: &mut Vec<u32>) -> Result<(usize, u32), &'static str> {
    let index = elves
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(idx, _)| idx)
        .ok_or("Can't find the elf index with maximum calories!")?;

    let value = elves
        .iter()
        .max()
        .ok_or("Cannot find the value of maximum calories!")?;

    Ok((index, *value))
}

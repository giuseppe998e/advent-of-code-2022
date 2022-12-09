pub fn part_two(elves: &mut Vec<u32>) -> u32 {
    elves.sort_by(|a, b| b.cmp(a));
    elves.truncate(3);

    elves.iter().sum()
}

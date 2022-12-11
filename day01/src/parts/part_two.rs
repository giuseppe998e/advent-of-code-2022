pub fn part_two(elves: &mut [u32]) -> u32 {
    elves.sort_unstable_by(|a, b| b.cmp(a));
    elves[..3].iter().sum()
}

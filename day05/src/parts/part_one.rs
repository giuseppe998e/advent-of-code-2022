use crate::prelude::*;

pub fn part_one(stacks: &mut [Stack], procedures: &[Procedure]) -> Result<String> {
    for (quantity, from, to) in procedures.iter().copied() {
        let entries = {
            let stack_from = stacks
                .get_mut(from)
                .ok_or("There is no stack with the given \"from\" index!")?;

            (0..quantity)
                .map(|_| {
                    stack_from
                        .pop()
                        .ok_or("There are not enough entries in the \"from\" pile!")
                })
                .collect::<Result<Vec<char>>>()?
        };

        let stack_to = stacks
            .get_mut(to)
            .ok_or("There is no stack with the given \"to\" index!")?;

        entries.iter().for_each(|&entry| stack_to.push(entry));
    }

    let stacks_top = stacks
        .iter()
        .map(|stack| stack.last().copied().unwrap())
        .collect();

    Ok(stacks_top)
}

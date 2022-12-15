use crate::util::{index_transpose, iter_transpose};

fn trees_ahead_visibility<'a, T, I>(
    forest: &mut T,
    forest_len: usize,
    columns: usize,
) -> Box<[bool]>
where
    I: PartialOrd + Copy + 'a,
    T: Iterator<Item = &'a I>,
{
    let mut results = Vec::<bool>::with_capacity(forest_len);
    let mut stack = Vec::<(usize, &'a I)>::with_capacity(columns);

    let mut forest_enumerate = forest.enumerate();
    let forest_first = forest_enumerate.next().unwrap();

    results.push(true);
    stack.push(forest_first);

    for (index, tree) in forest_enumerate {
        if index % columns == 0 {
            stack.clear();
        }

        while let Some((stack_index, _)) =
            stack.last().filter(|(_, stack_tree)| *stack_tree <= tree)
        {
            results[*stack_index] = false;
            stack.pop();
        }

        results.push(true);
        stack.push((index, tree));
    }

    results.into_boxed_slice()
}

fn trees_backward_visibility<'a, T, I>(
    forest: &mut T,
    forest_len: usize,
    columns: usize,
) -> Box<[bool]>
where
    I: PartialOrd + Copy + 'a,
    T: Iterator<Item = &'a I>,
{
    let mut results = Vec::<bool>::with_capacity(forest_len);
    let mut stack = Vec::<(usize, &'a I)>::with_capacity(columns);

    let mut forest_enumerate = forest.enumerate();
    let forest_first = forest_enumerate.next().unwrap();

    results.push(true);
    stack.push(forest_first);

    for (index, tree) in forest_enumerate {
        if index % columns == 0 {
            stack.clear();
        }

        while stack
            .last()
            .filter(|(_, stack_tree)| *stack_tree < tree)
            .is_some()
        {
            stack.pop();
        }

        let visible = stack.is_empty();
        results.push(visible);

        stack.push((index, tree));
    }

    results.into_boxed_slice()
}

pub fn trees_horizontal_visibility(forest: &[u32], columns: usize) -> (Box<[bool]>, Box<[bool]>) {
    let forest_len = forest.len();

    let left_visibility = {
        let mut forest_iter = forest.iter();
        trees_backward_visibility(&mut forest_iter, forest_len, columns)
    };

    let right_visibility = {
        let mut forest_iter = forest.iter();
        trees_ahead_visibility(&mut forest_iter, forest_len, columns)
    };

    (left_visibility, right_visibility)
}

pub fn trees_vertical_visibility(forest: &[u32], columns: usize) -> (Box<[bool]>, Box<[bool]>) {
    let forest_len = forest.len();

    let top_visibility = {
        let mut forest_iter = iter_transpose(forest, columns);
        trees_backward_visibility(&mut forest_iter, forest_len, columns)
    };

    let bottom_visibility = {
        let mut forest_iter = iter_transpose(forest, columns);
        trees_ahead_visibility(&mut forest_iter, forest_len, columns)
    };

    (top_visibility, bottom_visibility)
}

pub fn part_one(forest: &[u32], columns: usize) -> usize {
    let (left_visibility, right_visibility) = trees_horizontal_visibility(forest, columns);
    let (top_visibility, bottom_visibility) = trees_vertical_visibility(forest, columns);

    (0..forest.len())
        .filter(|&index| {
            let left_visible = left_visibility[index];
            let right_visible = right_visibility[index];

            if left_visible || right_visible {
                return true;
            }

            let vertical_index = index_transpose(index, columns);
            top_visibility[vertical_index] || bottom_visibility[vertical_index]
        })
        .count()
}

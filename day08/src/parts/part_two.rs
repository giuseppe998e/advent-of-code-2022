use crate::util::{index_transpose, iter_transpose};

// Modified "Next Greater Element" algorithm
fn trees_ahead_view_distance<'a, T, I>(
    forest: &mut T,
    forest_len: usize,
    columns: usize,
) -> Box<[usize]>
where
    I: PartialOrd + Copy + 'a,
    T: Iterator<Item = &'a I>,
{
    let mut results = Vec::<usize>::with_capacity(forest_len);
    let mut stack = Vec::<(usize, &'a I)>::with_capacity(columns);

    let mut forest_enumerate = forest.enumerate();
    let forest_first = forest_enumerate.next().unwrap();

    results.push(0);
    stack.push(forest_first);

    for (index, tree) in forest_enumerate {
        let column = index % columns;

        if column == 0 {
            stack.clear();
        }

        while let Some((stack_index, _)) =
            stack.last().filter(|(_, &stack_tree)| stack_tree <= *tree)
        {
            results[*stack_index] = index - stack_index;
            stack.pop();
        }

        results.push(columns - (column) - 1);
        stack.push((index, tree));
    }

    results.into_boxed_slice()
}

// Modified "Next Greater Element" algorithm
fn trees_backward_view_distance<'a, T, I>(
    forest: &mut T,
    forest_len: usize,
    columns: usize,
) -> Box<[usize]>
where
    I: PartialOrd + Copy + 'a,
    T: Iterator<Item = &'a I>,
{
    let mut results = Vec::<usize>::with_capacity(forest_len);
    let mut stack = Vec::<(usize, &'a I)>::with_capacity(columns);

    let mut forest_enumerate = forest.enumerate();
    let forest_first = forest_enumerate.next().unwrap();

    results.push(0);
    stack.push(forest_first);

    for (index, tree) in forest_enumerate {
        let column = index % columns;

        if column == 0 {
            stack.clear();
        }

        while stack
            .last()
            .filter(|(_, &stack_tree)| stack_tree < *tree)
            .is_some()
        {
            stack.pop();
        }

        let view_distance = stack
            .last()
            .map(|(stack_index, _)| index - stack_index)
            .unwrap_or(column);
        results.push(view_distance);

        stack.push((index, tree));
    }

    results.into_boxed_slice()
}

pub fn trees_horizontal_view_distance(
    forest: &[u32],
    columns: usize,
) -> (Box<[usize]>, Box<[usize]>) {
    let forest_len = forest.len();

    let left_view_distance = {
        let mut forest_iter = forest.iter();
        trees_backward_view_distance(&mut forest_iter, forest_len, columns)
    };

    let right_view_distance = {
        let mut forest_iter = forest.iter();
        trees_ahead_view_distance(&mut forest_iter, forest_len, columns)
    };

    (left_view_distance, right_view_distance)
}

pub fn trees_vertical_view_distance(
    forest: &[u32],
    columns: usize,
) -> (Box<[usize]>, Box<[usize]>) {
    let forest_len = forest.len();

    let top_view_distance = {
        let mut forest_iter = iter_transpose(forest, columns);
        trees_backward_view_distance(&mut forest_iter, forest_len, columns)
    };

    let bottom_view_distance = {
        let mut forest_iter = iter_transpose(forest, columns);
        trees_ahead_view_distance(&mut forest_iter, forest_len, columns)
    };

    (top_view_distance, bottom_view_distance)
}

pub fn part_two(forest: &[u32], columns: usize) -> usize {
    let (left_view_distance, right_view_distance) = trees_horizontal_view_distance(forest, columns);
    let (top_view_distance, bottom_view_distance) = trees_vertical_view_distance(forest, columns);

    (0..forest.len())
        .map(|index| {
            let vertical_index = index_transpose(index, columns);

            left_view_distance[index]
                * right_view_distance[index]
                * top_view_distance[vertical_index]
                * bottom_view_distance[vertical_index]
        })
        .max()
        .unwrap_or_default()
}

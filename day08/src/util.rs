#[inline]
pub fn index_transpose(index: usize, columns: usize) -> usize {
    let row = index / columns;
    let column = index % columns;

    column * columns + row
}

pub fn iter_transpose<T>(slice: &[T], columns: usize) -> impl Iterator<Item = &T> {
    (0..slice.len()).map(move |index| {
        let index = index_transpose(index, columns);
        &slice[index]
    })
}

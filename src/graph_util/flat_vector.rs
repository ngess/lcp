pub fn get_index(row: usize, col: usize, cols: usize)->usize{
    row * cols + col
}

pub fn index_to_coord(index: usize, cols: usize)->[usize;2]{
    [
        index/cols,
        index%cols
    ]
}
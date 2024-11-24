fn get_index(row: usize, col: usize, col_size: usize)->usize{
    row * col_size + col
}

fn index_to_coord(index: usize, col_size: usize)->[usize;2]{
    [
        index/col_size,
        index%col_size
    ]
}
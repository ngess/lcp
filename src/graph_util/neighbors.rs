use crate::graph_util::flat_vector::{index_to_coord,get_index};
use rayon::prelude::*;

pub fn compute_neighbors(index: &[usize; 2], rows: usize, cols: usize)->Vec<[usize;2]>{
    let delta: [(i32, i32);8] = [(-1, 0), (-1, 1), (0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1)];
    
    delta.iter().filter_map(|(delta_x, delta_y)|{
        let x: usize = (index[1] as i32 +delta_x) as usize;
        let y = (index[0] as i32 + delta_y) as usize;
        if x >= 0 && x < cols && y >= 0 && y < rows{
            return Some([y,x])
        }
        None
    }).collect()
}


// TODO: Multithreaded generation of hashmap
pub fn create_neighbor_list(raster: Vec<u32>, rows: usize, cols: usize)->Vec<Vec<(usize, u32)>>{
    let mut neighbor_vec = vec![Vec::new();raster.len()];
    neighbor_vec
        .par_iter_mut() // Use mutable parallel iterator
        .enumerate()    // To get the index for each element
        .for_each(|(index, neighbors)| {
            let coords = index_to_coord(index, cols);
            let neighbor_coords = compute_neighbors(&coords, rows, cols);
            let value = raster[index];
            *neighbors = neighbor_coords
                .into_iter()
                .map(|[row, col]| {
                    let neighbor_index = get_index(row, col, cols);
                    (neighbor_index, value)
                })
                .collect();
    });    
    neighbor_vec
}
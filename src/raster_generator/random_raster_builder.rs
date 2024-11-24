use ndarray::Array2;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use rand::{Rng, RngCore, SeedableRng};
use rand::rngs::StdRng;

pub fn create_random_array(array_size: &[usize;2], max_value: u32)->Array2<u32>{
    let mut rng = StdRng::seed_from_u64(10);
    Array2::<u32>::random_using((array_size[0], array_size[1]), Uniform::new(1,max_value+1), &mut rng)
}



pub fn create_random_vector_cost_raster(array_size: &[usize;2], max_value: u32)->Vec<Vec<u32>>{
    let mut rng = StdRng::seed_from_u64(10);
  
    let ret = (0..array_size[0]).map(
        |_|{
            (0..array_size[1]).map(
                |_|{
                    rng.gen_range(1..=max_value)
                }
            ).collect()
        }
    ).collect();
    ret
}

pub fn create_random_flat_cost_raster(array_size: &[usize;2], max_value: u32)-> Vec<u32>{
    let mut rng = StdRng::seed_from_u64(10);
    let ret = (0..array_size[0] * array_size[1]).map(
        |_|{
            rng.gen_range(1..=max_value)
        }
    ).collect();
    ret
}

use ndarray::Array2;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub fn create_random_array(array_size: &[usize;2], max_value: u32)->Array2<u32>{
    let mut rng = StdRng::seed_from_u64(10);
    Array2::<u32>::random_using((array_size[0], array_size[1]), Uniform::new(1,max_value+1), &mut rng)
}


pub mod data_model{
    pub mod input_model;
}
pub mod raster_generator{
    pub mod random_raster_builder;
}


use clap::Parser;
use data_model::input_model::SimulationArgs;
use raster_generator::random_raster_builder::create_random_flat_cost_raster;
pub mod graph_util{
    pub mod flat_vector;
    pub mod neighbors;
}

use graph_util::neighbors::create_neighbor_list;

use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(name = "LCP")]
#[command(version, about)]
struct Args{
    #[arg(short='c',long="config_path")]
    config_path: Option<String>,
}



fn main() {
    let cli_args = Args::parse();
    let simulation_args = match cli_args.config_path{
        Some(config_path)=>{
            SimulationArgs::read_from_file(config_path).unwrap()
        },
        None=>{
            SimulationArgs::new([20000,20000], 2, 2).unwrap()
        }
    };
    println!("Simulation Args: {:?}", simulation_args);
    let raster = create_random_flat_cost_raster(&simulation_args.raster_size, simulation_args.max_value);
    // println!("{:?}", raster);
    let start: Instant = Instant::now();
    let neighbor_list = create_neighbor_list(raster, simulation_args.raster_size[0], simulation_args.raster_size[1]);
    let duration: Duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

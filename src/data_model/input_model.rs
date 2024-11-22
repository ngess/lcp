use std::{fmt::Debug, fs::File, io::BufReader};
use serde::{Serialize, Deserialize};
use serde_yaml::Error;
use clap::builder::Str;
use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InputModel{
    pub input_dataset_path: Option<String>,
    pub output_datsaet_path: Option<String>
}


impl InputModel{
    // TODO: Yaml Parse
    pub fn read_from_file(file_path: Str)->Result<InputModel, Error>{
        let input_file = File::open(file_path).unwrap();
        let reader = BufReader::new(input_file);
        serde_yaml::from_reader(reader)
    }
    
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SimulationArgs{
    pub raster_size: [usize;2],
    pub point_pairs: Vec<PointPair>,
    pub max_value: u32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PointPair{
    start_point: Point,
    end_point: Point
}

impl PointPair{
    pub fn new(start_point: Point, end_point: Point)->PointPair{
        PointPair{
            start_point,
            end_point
        }
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Point{
    id: String,
    x: usize,
    y: usize
}

impl Point{
    pub fn new(id: String, x: usize, y: usize)->Point{
        Point{
            id,
            x,
            y
        }
    }

    // pub fn new_random_point_in_range(id: String, x_min: usize, y_min: usize, x_max: usize, y_max: usize)->Point{
    //     Point{
    //         id,

    //     }
    // }
}


impl SimulationArgs{
    // TODO: Yaml Parse
    pub fn read_from_file(file_path: String)->Result<SimulationArgs, Error>{
        let input_file = File::open(file_path).unwrap();
        let reader = BufReader::new(input_file);
        serde_yaml::from_reader(reader)
    }

    fn _create_new_points(rng: &mut StdRng, route_number: usize, raster_size: [usize;2])->PointPair{
        PointPair::new(
            Point::new(
                format!("Route_{}_Point_1", route_number),
                rng.gen_range(0..raster_size[0]),
                rng.gen_range(0..raster_size[1])
            ),
            Point::new(
                format!("Route_{}_Point_2", route_number),
                rng.gen_range(0..raster_size[0]),
                rng.gen_range(0..raster_size[1])
            )
        )

    }

    pub fn new(raster_size: [usize;2], total_routes: usize, max_value: u32)->Result<SimulationArgs, Box<dyn std::error::Error>>{
        if raster_size[0] < 1 && raster_size[1] < 1 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid raster size, dimensions must be greater than 1"
            )));        
        }
        if total_routes < 1 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Total routes must be greater than 1"
            )));        
        }
        

        let mut point_pairs: Vec<PointPair> = Vec::new();
        let mut rng = StdRng::seed_from_u64(4);
        for i in 0..total_routes{            
            point_pairs.push(
                SimulationArgs::_create_new_points(&mut rng, i, raster_size)
            );
        }
        Ok(SimulationArgs{
            raster_size,
            point_pairs,
            max_value
        })

    }
    
}
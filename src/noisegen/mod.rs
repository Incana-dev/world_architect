pub mod plate;

use std::collections::HashMap;

use crate::config::{PERLIN_SCALE, PLATE_DISTORTION_STRENGTH, VORONOI_SCALE, WORLD_SIZE};
use crate::noisegen::plate::TectonicPlateType;
use crate::world;
use noise::core::worley::ReturnType;
use noise::{Fbm, NoiseFn, Perlin, Worley};
use rand::seq::IndexedRandom;
use std::collections::HashSet;

#[derive(Default)]
pub struct WorldNoiseMap {
    noise_instance: Vec<f64>,
}

// Voronoi noise region
pub struct PlateData {
    pub id: f64,
    pub dist_to_edge: f64,
}

pub struct WorldPlateMap {
    pub plates: Vec<PlateData>,
    pub plate_data: HashMap<u32, TectonicPlateType>
}

impl WorldPlateMap {
    pub fn get_plate_type(&self, index: usize) -> TectonicPlateType {
        let tile_plate_data = &self.plates[index];
        let plate_id_key = tile_plate_data.id as u32;
        let plate_type = *self.plate_data.get(&plate_id_key).unwrap();
        plate_type
    }

    pub fn get_distance_to_edge(&self, index: usize) -> f64 {
        self.plates[index].dist_to_edge
    }
}

impl WorldNoiseMap {
    pub fn get_height_from_noisemap(&self, index: usize) -> u8{
        // clamp the f64 at noise_instance[index] to u8
        let noise_value = self.noise_instance[index as usize];
        let mapped_value = ((noise_value + 1.0) * 127.5) as u8;
        mapped_value
    }
}

pub fn gen_elevation_map(seed: u32) -> WorldNoiseMap{

    let scale = PERLIN_SCALE;


    let mut nm = WorldNoiseMap {
        noise_instance: Vec::new(),
    };


    let fbm = Fbm::<Perlin>::new(seed);
 
    for cell in 0..WORLD_SIZE {
        let (x, y) = world::get_coords_from_index(cell as usize);
        // let point:[f64; 2] = [x as f64, y as f64];
        let point_for_noise = [
            x as f64 / scale,
            y as f64 / scale,
        ];
        nm.noise_instance.push(fbm.get(point_for_noise));
    }
    nm
}



pub fn gen_plate_map(seed: u32) -> WorldPlateMap {
    let id_generator = Worley::new(seed).set_return_type(ReturnType::Value);
    let dist_generator = Worley::new(seed).set_return_type(ReturnType::Distance);

    let distortion_generator = Fbm::<Perlin>::new(seed);

    let mut world_plates = WorldPlateMap {
        plates: Vec::with_capacity(WORLD_SIZE as usize),
        plate_data: HashMap::new(),
    };

    for cell in 0..(WORLD_SIZE as usize) {
        let (x, y) = world::get_coords_from_index(cell);
        
        let point = [
            x as f64 / VORONOI_SCALE,
            y as f64 / VORONOI_SCALE
        ];

        // Apply Domain Warping. This is complex, and not something I fully understand yet. Though I intend to write a blogpost on it later.
        // https://iquilezles.org/articles/warp/
        let x_offset = distortion_generator.get([point[0] * 2.0, point[1] * 2.0]);
        let y_offset = distortion_generator.get([point[0] * 2.0 + 5.3, point[1] * 2.0 - 1.7]);


        let warped_point = [
            point[0] + (x_offset * PLATE_DISTORTION_STRENGTH),
            point[1] + (y_offset * PLATE_DISTORTION_STRENGTH),
        ];

        // 4. Use a warped point to query the Worley generators.
        world_plates.plates.push(PlateData {
            id: id_generator.get(warped_point),
            dist_to_edge: dist_generator.get(warped_point),
        });
    }

    let plate_types: &[TectonicPlateType] = &[
        TectonicPlateType::Oceanic,
        TectonicPlateType::Continental,
    ];

    let unique_ids: HashSet<u32> = world_plates
        .plates
        .iter()
        .map(|p| p.id as u32)
        .collect();

    for id in unique_ids {
        let mut rng = rand::thread_rng();
        let random_plate_type = plate_types.choose(&mut rng).unwrap();
        world_plates.plate_data.insert(id, *random_plate_type);
    }

    world_plates
}
use crate::config::{WORLD_X_WIDTH, WORLD_Y_HEIGHT};
use crate::world;
use noise::{Fbm, NoiseFn, Perlin};

#[derive(Default)]
pub struct WorldNoiseMap {
    noise_instance: Vec<f64>,
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

    let scale = 6.0;


    let mut nm = WorldNoiseMap {
        noise_instance: Vec::new(),
    };

    let x = WORLD_X_WIDTH;
    let y = WORLD_Y_HEIGHT;

    let world_cell_count = x * y;

    let fbm = Fbm::<Perlin>::new(seed);
 
    for cell in 0..world_cell_count {
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

pub mod tile;

use std::ops::Index;

use crate::world::tile::TileType;
use crate::{world::tile::Tile};
use crate::config::{self, *};
use crate::noisegen::{self, gen_elevation_map, gen_plate_map, WorldPlateMap};
use macroquad::color::{Color, BLACK};
use crate::noisegen::plate::*;

use macroquad::prelude::*;
use crate::noisegen::WorldNoiseMap;

#[derive(Debug)]
pub struct World {
    // contains every tile in the world. Can contain other logic later.
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
    pub draw_texture: Option<macroquad::texture::RenderTarget>,

    
}

impl World {

    fn new(width: u32, height: u32) -> Self {
        let world_size = (width * height) as usize;
        // Create the Vec of default tiles
        let tiles = vec![Tile::default(); world_size];
        // Return a new instance of the struct
        let mut world_inst = Self {
            tiles,
            width,
            height,
            draw_texture: None,
        };

        world_inst.generate_terrain();
        

        world_inst
    }




    fn for_each_tile<F>(&mut self, mut operation: F)
        where
            F: FnMut(&mut Tile, usize), // signature of the operation I'm passing
        {
            for (index, tile) in self.tiles.iter_mut().enumerate() {
                operation(tile, index); 
            }
        }

fn generate_terrain(&mut self) {
    // Perlin noise map
    let base_elevation_map = gen_elevation_map(config::WORLD_SEED as u32);

    // Voronoi/Worley based plates
    let plate_map = gen_plate_map(config::WORLD_SEED as u32);

    self.for_each_tile(|tile, index| {
            // --- Start all calculations with f64 ---
            // 1. Get the base height from Perlin noise.
            let base_height = base_elevation_map.get_height_from_noisemap(index) as f64;

            // 2. Get the plate type for this tile.
            let plate_type = plate_map.get_plate_type(index);

            // 3. Start with the base height and add modifiers.
            let mut final_elevation = base_height;

            // 4. Add elevation based on plate type.
            if plate_type == TectonicPlateType::Continental {
                final_elevation += 20.0;
            } else {
                // It's fine to go below zero here, we'll clamp later.
                final_elevation -= 10.0;
            }

            // 5. Add mountains at plate boundaries.
            let distance_to_edge = plate_map.get_distance_to_edge(index);
            if distance_to_edge < config::MOUNTAIN_FORMATION_DISTANCE {
                let mountain_factor = 1.0 - (distance_to_edge / config::MOUNTAIN_FORMATION_DISTANCE);
                
                // Squaring the factor makes the mountains rise steeply from the edge.
                // A small factor (0.2) becomes much smaller (0.04), while a large
                // factor (0.9) stays large (0.81), creating a sharp curve.
                let mountain_bonus = config::MOUNTAIN_MAX_HEIGHT * mountain_factor.powf(2.0);

                final_elevation += mountain_bonus;
            }

            // --- Final Step: Clamp and cast to u8 ---
            // Clamp the final value to the valid 0-255 range before casting.
            tile.elevation = final_elevation.clamp(0.0, 255.0) as u8;
            
            // Now that the final elevation is set, update the tile's type and color.
            tile.update_tile_type();
            tile.update_color();
        });
    }

    fn apply_water_level(&mut self){
        self.for_each_tile(|tile, Index| {
            if tile.elevation < WATER_LEVEL {
                tile.tile_type = TileType::Ocean;
                tile.update_color();
            }
        });
    }



    fn get_world_tile(&mut self, x: u32, y: u32) -> Option<&mut Tile> {
        if x >= self.width || y >= self.height {
            None // Coordinates OOB
        } else {
            let index = (y * self.width + x) as usize;
            self.tiles.get_mut(index)
        }
    }

    fn build_render_target(&mut self) {
        let total_width = self.width as f32 * CELL_WIDTH as f32;
        let total_height = self.height as f32 * CELL_WIDTH as f32;

        let render_target = render_target(total_width as u32, total_height as u32);
        render_target.texture.set_filter(FilterMode::Linear);

        let mut render_target_cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, total_width, total_height));
        
        render_target_cam.render_target = Some(render_target.clone());

        set_camera(&render_target_cam);
        for n in 0..self.tiles.len() {
            // Determine the (x, y) coordinates of the tile in the grid
            let tile_x = (n as u32 % self.width) as f32;
            let tile_y = (n as u32 / self.width) as f32;

            let tile_color = self.tiles[n].tile_color;

            // Determine the pixel position on the screen
            let draw_x = tile_x * CELL_WIDTH as f32;
            let draw_y = tile_y * CELL_WIDTH as f32;


            draw_rectangle(
                draw_x,
                draw_y,
                CELL_WIDTH as f32,
                CELL_WIDTH as f32,
                tile_color,
            );
        }

        self.draw_texture = render_target_cam.render_target;

        set_default_camera();  
    }
}


pub fn generate_new_world(width: u32, height: u32) -> World {
    // Allocate cells.
    let mut world = World::new(width, height);

    world.build_render_target();

    world
}

pub fn get_coords_from_index( index: usize) -> (u32, u32){
    let width = config::WORLD_X_WIDTH as usize;

    let x = index % width;
    let y = index / width;

    (x as u32, y as u32)
}

pub fn get_index_from_coords( x: u32,y: u32) -> u32{
    let index = y * config::WORLD_X_WIDTH + x;

    index

}

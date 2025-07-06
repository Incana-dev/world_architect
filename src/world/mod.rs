
pub mod tile;

use std::ops::Index;

use crate::world::tile::TileType;
use crate::{world::tile::Tile};
use crate::config::{self, *};
use crate::noisegen::{self, gen_elevation_map};
use macroquad::color::{Color, BLACK};

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

        world_inst.apply_heightmap();
        world_inst.apply_water_level();

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

    fn apply_heightmap(&mut self){
        let initial_heightmap = gen_elevation_map(config::WORLD_SEED as u32);
        self.for_each_tile(|tile, index| {
            let height = initial_heightmap.get_height_from_noisemap(index);
            tile.elevation = height;
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

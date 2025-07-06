#![allow(unused_variables, unused_mut, dead_code)]

mod world;
mod config;
mod noisegen;


use macroquad::prelude::*;


#[macroquad::main("World Generator")]
async fn main() {
    
    let mut world_inst = world::generate_new_world(config::WORLD_X_WIDTH, config::WORLD_Y_HEIGHT);
    // println!("{:?}", world_inst);
        loop {
        clear_background(BLACK);


        if let Some(target) = &world_inst.draw_texture {
            draw_texture(&target.texture, 0.0, 0.0, WHITE);
        }

        next_frame().await;
    }

    

}

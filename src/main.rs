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


        draw_texture(
            &world_inst.draw_texture.as_ref().unwrap().texture, // The texture to draw
            0.0,  // X position on the screen
            0.0,  // Y position on the screen
            WHITE, // Color tint
        );

        next_frame().await;
    }

    

}

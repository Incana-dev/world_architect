use std::u8;

use bitflags::bitflags;
use macroquad::{color::{Color, BLACK}, telemetry};

use crate::config::{self, ROCK_LEVEL, WATER_LEVEL, SNOW_LEVEL};


#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub elevation: u8,
    properties: LandTileProperties,
    pub tile_color: Color,

}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::Land,
            elevation: 1,
            properties: LandTileProperties::empty(),
            tile_color: BLACK,
        }
    }
}

impl Tile {
    pub fn update_color(&mut self){

        self.tile_color = match self.tile_type {
            TileType::Ocean => {
                let shallows_factor = self.elevation as f32 / (WATER_LEVEL - 1) as f32;
                let blue_val = 50.0 + (150.0 * shallows_factor);
                Color::from_rgba(0, 50, blue_val as u8, 255)
            },
            TileType::Land => {
                let mut land_color = Color::from_rgba(0, 0, 0, 255);
                match self.elevation {
                    WATER_LEVEL..ROCK_LEVEL => {
                        let lowland = (self.elevation as f32 - WATER_LEVEL as f32) /
                        (ROCK_LEVEL as f32 - WATER_LEVEL as f32);
                        let green_val = 120.0 + 60.0 * lowland;
                        Color::from_rgba(30, green_val as u8, 30, 255)
                    },
                    ROCK_LEVEL..SNOW_LEVEL => {
                        Color::from_rgba(105, 105, 105, 255)
                    },
                    SNOW_LEVEL..=u8::MAX => {
                        Color::from_rgba(255, 255, 255, 255)
                    }
                    _ => {Color::from_rgba(255,0,0,255)}
                }
                
            }
        };

    }
}

#[derive(Debug, Clone)]
pub enum TileType {
    Land,
    Ocean,
}



bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LandTileProperties: u32 {
        const HasRiver = 0b00000001;
        const HasMountain = 0b00000010;
        const HasForest = 0b00000100;
        const HasSettlement = 0b00001000;
        const HasFarmland = 0b00010000;
    }

}


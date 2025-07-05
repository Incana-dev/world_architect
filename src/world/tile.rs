use bitflags::bitflags;
use macroquad::color::{Color, BLACK};


#[derive(Debug, Clone)]
pub struct Tile {
    tile_type: TileType,
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
        self.tile_color = Color::from_rgba(255, 255, 255, self.elevation);
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


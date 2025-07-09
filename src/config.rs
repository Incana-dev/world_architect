// This will contain world generation rules
pub const WORLD_X_WIDTH: u32 = 256;
pub const WORLD_Y_HEIGHT: u32 = 256;
pub const WORLD_SIZE: u32 = WORLD_X_WIDTH * WORLD_Y_HEIGHT;

pub const CELL_WIDTH: u8 = 4;
pub const WORLD_SEED: usize = 28282828;
pub const WATER_LEVEL: u8 = 130;
pub const ROCK_LEVEL: u8 = 190;
pub const SNOW_LEVEL: u8 = 215;


pub const MOUNTAIN_FORMATION_DISTANCE: f64 = 0.2; 
pub const MOUNTAIN_MAX_HEIGHT: f64 = 1.80;

pub const PERLIN_SCALE: f64 = 26.0;
pub const VORONOI_SCALE: f64 = 76.0;

pub const PLATE_DISTORTION_STRENGTH: f64 = 4.0;
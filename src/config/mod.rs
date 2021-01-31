pub mod log;

use super::{error, utils};

pub const APPLICATION_NAME: &str = "Rust Game Prototype";
pub const APPLICATION_ID: &str = env!("CARGO_PKG_NAME");
pub const APPLICATION_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

#[cfg(not(debug_assertions))]
pub const LOGS_DIR_NAME: &str = ".logs";

#[cfg(not(debug_assertions))]
pub const LOG_FILE_EXT: &str = "log";

pub const ASSETS_PATH: &str = "assets";

pub const SETTINGS_FILE_PATH: &str = "/settings.conf";

pub const APPLICATION_ICON_FILE_PATH: &str = "/icon.png";

pub const WALK_SECONDS_PER_TILE: f32 = 0.25;
pub const WAIT_AFTER_TURN_BEFORE_MOVE: f32 = 0.175;

// 16x16 tile sizes
pub const TILE_PIXELS_SIZE_USIZE: usize = 16;
pub const TILE_PIXELS_SIZE_F32: f32 = TILE_PIXELS_SIZE_USIZE as f32;

// I like 19x11 so far better than 15x9
pub const VIEWPORT_TILES_WIDTH_USIZE: usize = 19;
pub const VIEWPORT_TILES_WIDTH_F32: f32 = VIEWPORT_TILES_WIDTH_USIZE as f32;

pub const VIEWPORT_TILES_HEIGHT_USIZE: usize = 11;
pub const VIEWPORT_TILES_HEIGHT_F32: f32 = VIEWPORT_TILES_HEIGHT_USIZE as f32;

pub const VIEWPORT_PIXELS_WIDTH_USIZE: usize = VIEWPORT_TILES_WIDTH_USIZE * TILE_PIXELS_SIZE_USIZE;
pub const VIEWPORT_PIXELS_WIDTH_F32: f32 = VIEWPORT_PIXELS_WIDTH_USIZE as f32;

pub const VIEWPORT_PIXELS_HEIGHT_USIZE: usize =
    VIEWPORT_TILES_HEIGHT_USIZE * TILE_PIXELS_SIZE_USIZE;
pub const VIEWPORT_PIXELS_HEIGHT_F32: f32 = VIEWPORT_PIXELS_HEIGHT_USIZE as f32;

pub const ENTITY_SPRITE_SHEET_IDX_IDLE_DOWN: usize = 0;
pub const ENTITY_SPRITE_SHEET_IDX_IDLE_RIGHT: usize = 1;
pub const ENTITY_SPRITE_SHEET_IDX_IDLE_UP: usize = 2;
pub const ENTITY_SPRITE_SHEET_IDX_IDLE_LEFT: usize = 3;

pub const ENTITY_SPRITE_SHEET_IDX_WALK_DOWN: usize = 4;
pub const ENTITY_SPRITE_SHEET_IDX_WALK_RIGHT: usize = 5;
pub const ENTITY_SPRITE_SHEET_IDX_WALK_UP: usize = 6;
pub const ENTITY_SPRITE_SHEET_IDX_WALK_LEFT: usize = 7;

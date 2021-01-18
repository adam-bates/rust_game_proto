use super::error;
use super::utils;

pub mod log;

pub const APPLICATION_NAME: &str = "Rust Game Prototype";
pub const APPLICATION_ID: &str = env!("CARGO_PKG_NAME");
pub const APPLICATION_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub const LOGS_DIR_NAME: &str = ".logs";
pub const LOG_FILE_EXT: &str = "log";

pub const SETTINGS_FILE_PATH: &str = "/settings.conf";

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

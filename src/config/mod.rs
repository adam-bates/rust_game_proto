use super::error;
use super::utils;

pub mod log;

pub const APPLICATION_NAME: &str = "Rust Game Prototype";
pub const APPLICATION_ID: &str = env!("CARGO_PKG_NAME");
pub const APPLICATION_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub const LOGS_DIR_NAME: &str = ".logs";
pub const LOG_FILE_EXT: &str = "log";

pub const SETTINGS_FILE_PATH: &str = "/settings.conf";

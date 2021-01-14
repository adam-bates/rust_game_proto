pub mod serialize;
pub mod types;

pub use types::Settings;

use super::error::types::GameResult;
use serialize::{load_settings, save_settings};
use types::build_default_settings;

pub fn find_or_default_for_user(fs: &mut ggez::filesystem::Filesystem) -> GameResult<Settings> {
    // Try to load settings
    if let Some(settings) = load_settings(fs) {
        return Ok(settings);
    };

    // If no settings, then build, save, and return default settings
    let default_settings = build_default_settings();
    save_settings(fs, &default_settings)?;
    Ok(default_settings)
}

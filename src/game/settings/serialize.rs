use super::GameResult;
use super::Settings;

pub fn load_settings(fs: &mut ggez::filesystem::Filesystem) -> Option<Settings> {
    None // TODO
}

pub fn save_settings(fs: &mut ggez::filesystem::Filesystem, settings: &Settings) -> GameResult {
    Ok(()) // TODO
}

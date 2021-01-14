use super::{filesystem, GameResult, Settings};
use ggez::error::GameError;
use std::path::Path;

const SETTINGS_PATH: &str = "/settings.conf";

fn find_config_vfs(fs: &ggez::filesystem::Filesystem) -> GameResult<&Box<dyn ggez::vfs::VFS>> {
    fs.find_vfs(&fs.user_config_path)
        .ok_or_else(|| GameError::ConfigError("Failed to find user config vfs".to_string()))
}

pub fn load_settings(fs: &mut ggez::filesystem::Filesystem) -> GameResult<Option<Settings>> {
    let config_vfs = find_config_vfs(fs)?;
    let settings_path = Path::new(SETTINGS_PATH);

    if !filesystem::file_exists(config_vfs, settings_path) {
        return Ok(None);
    }

    let mut file = config_vfs.open(settings_path)?;
    let c = Settings::from_toml_file(&mut file)?;
    Ok(Some(c))
}

pub fn save_settings(fs: &mut ggez::filesystem::Filesystem, settings: &Settings) -> GameResult {
    let config_vfs = find_config_vfs(fs)?;
    let settings_path = Path::new(SETTINGS_PATH);

    let mut file = filesystem::create_file(config_vfs, settings_path)?;
    settings.to_toml_file(&mut file)?;

    if !fs.is_file(settings_path) {
        return Err(GameError::ConfigError(format!(
            "Failed to write settings file at {}",
            settings_path.to_string_lossy()
        )));
    }

    Ok(())
}

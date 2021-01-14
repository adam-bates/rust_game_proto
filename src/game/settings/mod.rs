pub mod serialize;
pub mod types;

pub use types::Settings;

use super::{config, error::types::GameResult, filesystem};
use serialize::{load_settings, save_settings};

pub fn find_or_default_for_user(
    fs: &mut ggez::filesystem::Filesystem,
) -> GameResult<(Settings, bool)> {
    match load_settings(fs)? {
        Some(settings) => Ok((settings, false)),
        _ => Ok((Settings::default(), true)),
    }
}

pub fn initialize_first_load(ctx: &mut ggez::Context, user_settings: &mut Settings) -> GameResult {
    // TODO: Update settings using context (ie. width / height from monitor size)

    save_settings(&mut ctx.filesystem, user_settings)
}

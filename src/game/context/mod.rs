use super::{error::types::GameResult, settings::Settings};

mod factory;

pub fn new_context(
    fs: ggez::filesystem::Filesystem,
    user_settings: &Settings,
) -> GameResult<(ggez::Context, ggez::event::EventLoop<()>)> {
    factory::new_context(fs, user_settings)
}

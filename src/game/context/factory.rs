use super::{GameResult, Settings};

pub fn new_context(
    fs: ggez::filesystem::Filesystem,
    user_settings: &Settings,
) -> GameResult<(ggez::Context, ggez::event::EventsLoop)> {
    ggez::Context::from_conf(user_settings.into(), fs)
}

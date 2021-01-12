use super::config;
use super::settings::Settings;

mod factory;

pub fn new_context(
    _fs: ggez::filesystem::Filesystem,
    _user_settings: &Settings,
) -> (ggez::Context, ggez::event::EventsLoop) {
    factory::new_context()
}

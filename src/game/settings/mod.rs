pub mod serialize;
pub mod types;

pub use types::{AspectRatio, Settings};

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

fn set_window_to_half_resolution(
    ctx: &mut ggez::Context,
    user_settings: &mut Settings,
) -> GameResult {
    let window = ggez::graphics::window(ctx);
    let monitor = window.get_current_monitor();
    let monitor_dimensions = monitor.get_dimensions();

    let hidpi_factor = monitor.get_hidpi_factor();

    let max_resolution = (
        (monitor_dimensions.width / hidpi_factor) as f32,
        (monitor_dimensions.height / hidpi_factor) as f32,
    );

    user_settings.video_settings.windowed_width = (max_resolution.0 / 2.) as usize;
    user_settings.video_settings.windowed_height = (max_resolution.1 / 2.) as usize;

    ggez::graphics::set_drawable_size(
        ctx,
        user_settings.video_settings.windowed_width as f32,
        user_settings.video_settings.windowed_height as f32,
    )?;

    Ok(())
}

pub fn initialize_first_load(ctx: &mut ggez::Context, user_settings: &mut Settings) -> GameResult {
    set_window_to_half_resolution(ctx, user_settings)?;
    save_settings(&mut ctx.filesystem, user_settings)?;

    Ok(())
}

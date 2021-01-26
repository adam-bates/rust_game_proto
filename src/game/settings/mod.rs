pub mod serialize;
pub mod types;

pub use types::{AspectRatio, Settings};

use super::{config, error::types::GameResult, filesystem, input};
use serialize::{load_settings, save_settings};

pub fn find_or_default_for_user(
    fs: &mut ggez::filesystem::Filesystem,
) -> GameResult<(Settings, bool)> {
    match load_settings(fs)? {
        Some(settings) => Ok((settings, false)),
        _ => Ok((Settings::default(), true)),
    }
}

fn get_current_monitor(ctx: &mut ggez::Context) -> GameResult<winit::monitor::MonitorHandle> {
    let window = ggez::graphics::window(ctx);
    window
        .current_monitor()
        .ok_or_else(|| ggez::GameError::VideoError("Couldn't find current monitor".to_string()))
}

fn set_window_to_half_resolution(
    ctx: &mut ggez::Context,
    user_settings: &mut Settings,
) -> GameResult {
    let monitor = get_current_monitor(ctx)?;
    let monitor_dimensions = monitor.size();

    let hidpi_factor = monitor.scale_factor();

    let max_resolution = (
        (monitor_dimensions.width as f64 / hidpi_factor) as f32,
        (monitor_dimensions.height as f64 / hidpi_factor) as f32,
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

fn set_fps_to_monitor_max(ctx: &mut ggez::Context, user_settings: &mut Settings) -> GameResult {
    let monitor = get_current_monitor(ctx)?;

    let max_monitor_fps = monitor.video_modes().map(|v| v.refresh_rate()).max();

    if let Some(max_monitor_fps) = max_monitor_fps {
        user_settings.video_settings.target_fps = max_monitor_fps as u32;
    }

    Ok(())
}

pub fn initialize_first_load(ctx: &mut ggez::Context, user_settings: &mut Settings) -> GameResult {
    set_window_to_half_resolution(ctx, user_settings)?;
    set_fps_to_monitor_max(ctx, user_settings)?;
    save_settings(&mut ctx.filesystem, user_settings)?;

    Ok(())
}

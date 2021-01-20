use super::{config, GameResult};
use ggez::conf::FullscreenType;
use serde_derive::{Deserialize, Serialize};
use std::io;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub enum AspectRatio {
    Stretch,

    #[serde(rename = "16:9")]
    Ratio16By9,

    #[serde(rename = "4:3")]
    Ratio4By3,

    #[serde(rename = "Pixel")]
    PixelPerfect,
}

// Resolution, window-mode (fullscreen, windowed, windowed-fullscreen), aspect ratio, color-blind mode
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoSettings {
    pub windowed_width: usize,
    pub windowed_height: usize,

    #[serde(rename = "window_mode")]
    pub fullscreen_type: FullscreenType,

    pub aspect_ratio: AspectRatio,

    pub target_fps: u32,

    pub vsync: bool,

    pub srgb: bool,
    // colour_blind_mode,
}

impl VideoSettings {
    fn apply(&self, ctx: &mut ggez::Context) -> GameResult {
        {
            if self.fullscreen_type == ggez::conf::FullscreenType::Windowed {
                ggez::graphics::set_drawable_size(
                    ctx,
                    self.windowed_width as f32,
                    self.windowed_height as f32,
                )?;
            }

            ggez::input::mouse::set_cursor_hidden(
                ctx,
                self.fullscreen_type == ggez::conf::FullscreenType::True,
            );
        }

        Ok(())
    }
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            windowed_width: config::VIEWPORT_PIXELS_WIDTH_USIZE,
            windowed_height: config::VIEWPORT_PIXELS_HEIGHT_USIZE,
            fullscreen_type: FullscreenType::Windowed,
            aspect_ratio: AspectRatio::Stretch,
            target_fps: 144, // TODO
            vsync: true,
            srgb: true,
        }
    }
}

// Controls (input mapping), locale, font, text-speed, ui-border-type
#[derive(Debug, Serialize, Deserialize)]
pub struct GameSettings {
    pub controller_stick_deadzone: f32,
    // input_mapping,
    // locale,
    // font,
    // text_speed,
    // ui_border_type,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            controller_stick_deadzone: 0.5,
        }
    }
}

impl GameSettings {
    fn apply(&self, ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }
}

// Volumes, sound-accessibility
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AudioSettings {
    // ...
}

impl AudioSettings {
    fn apply(&self, ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "game")]
    pub game_settings: GameSettings,

    #[serde(rename = "video")]
    pub video_settings: VideoSettings,

    #[serde(rename = "audio")]
    pub audio_settings: AudioSettings,
}

impl Settings {
    pub fn apply(&self, ctx: &mut ggez::Context) -> GameResult {
        {
            ggez::graphics::set_mode(ctx, self.into())?;
        }

        self.game_settings.apply(ctx)?;
        self.video_settings.apply(ctx)?;
        self.audio_settings.apply(ctx)?;

        Ok(())
    }

    pub fn from_toml_file<R: io::Read>(file: &mut R) -> GameResult<Self> {
        let mut encoded = String::new();
        file.read_to_string(&mut encoded)?;

        let settings = toml::from_str(&encoded)?;
        Ok(settings)
    }

    pub fn to_toml_file<W: io::Write>(&self, file: &mut W) -> GameResult {
        let buffer = toml::to_vec(self)?;
        file.write_all(&buffer)?;
        Ok(())
    }
}

impl Into<ggez::conf::WindowMode> for &Settings {
    fn into(self) -> ggez::conf::WindowMode {
        ggez::conf::WindowMode {
            fullscreen_type: self.video_settings.fullscreen_type,
            maximized: self.video_settings.fullscreen_type != ggez::conf::FullscreenType::Windowed,
            borderless: self.video_settings.fullscreen_type != ggez::conf::FullscreenType::Windowed,
            width: self.video_settings.windowed_width as f32,
            height: self.video_settings.windowed_height as f32,
            min_width: config::VIEWPORT_PIXELS_WIDTH_F32,
            min_height: config::VIEWPORT_PIXELS_HEIGHT_F32,
            max_width: 0.,
            max_height: 0.,
            resizable: true,
            visible: true,
        }
    }
}

impl Into<ggez::conf::WindowSetup> for &Settings {
    fn into(self) -> ggez::conf::WindowSetup {
        ggez::conf::WindowSetup {
            vsync: self.video_settings.vsync,
            srgb: self.video_settings.srgb,
            title: config::APPLICATION_NAME.to_string(),
            icon: "/background_pallet_town.png".to_string(), // TODO
            samples: ggez::conf::NumSamples::Zero,
        }
    }
}

impl Into<ggez::conf::Backend> for &Settings {
    fn into(self) -> ggez::conf::Backend {
        ggez::conf::Backend::default()
    }
}

impl Into<ggez::conf::ModuleConf> for &Settings {
    fn into(self) -> ggez::conf::ModuleConf {
        ggez::conf::ModuleConf {
            gamepad: true,
            audio: true,
        }
    }
}

impl Into<ggez::conf::Conf> for &Settings {
    fn into(self) -> ggez::conf::Conf {
        ggez::conf::Conf {
            window_mode: self.into(),
            window_setup: self.into(),
            backend: self.into(),
            modules: self.into(),
        }
    }
}

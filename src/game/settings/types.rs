use super::{config, GameResult};
use ggez::conf::FullscreenType;
use serde_derive::{Deserialize, Serialize};
use std::io;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub enum AspectRatio {
    #[serde(rename = "stretch")]
    Stretch,

    #[serde(rename = "16:9")]
    Ratio_16_9,

    #[serde(rename = "15:9")]
    Ratio_15_9,

    #[serde(rename = "4:3")]
    Ratio_4_3,
}

// Resolution, window-mode (fullscreen, windowed, windowed-fullscreen), aspect ratio, color-blind mode
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoSettings {
    pub window_width: usize,
    pub window_height: usize,
    pub window_mode: FullscreenType,
    pub aspect_ratio: AspectRatio,
    pub vsync: bool,
    pub srgb: bool,
    // colour_blind_mode,
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            window_width: config::VIEWPORT_PIXELS_WIDTH_USIZE,
            window_height: config::VIEWPORT_PIXELS_HEIGHT_USIZE,
            window_mode: FullscreenType::Windowed,
            aspect_ratio: AspectRatio::Stretch,
            vsync: true,
            srgb: true,
        }
    }
}

// Controls (input mapping), locale, font, text-speed, ui-border-type
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameSettings {
    // input_mapping,
// locale,
// font,
// text_speed,
// ui_border_type,
}

// Volumes, sound-accessibility
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AudioSettings {
    // ...
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

impl Into<ggez::conf::Conf> for &Settings {
    fn into(self) -> ggez::conf::Conf {
        ggez::conf::Conf {
            window_mode: ggez::conf::WindowMode {
                fullscreen_type: self.video_settings.window_mode,
                maximized: self.video_settings.window_mode != ggez::conf::FullscreenType::Windowed,
                borderless: self.video_settings.window_mode != ggez::conf::FullscreenType::Windowed,
                width: self.video_settings.window_width as f32,
                height: self.video_settings.window_height as f32,
                min_width: config::VIEWPORT_PIXELS_WIDTH_F32,
                min_height: config::VIEWPORT_PIXELS_HEIGHT_F32,
                max_width: 0.,
                max_height: 0.,
                resizable: true,
            },
            window_setup: ggez::conf::WindowSetup {
                vsync: self.video_settings.vsync,
                srgb: self.video_settings.srgb,
                title: "TITLE".to_string(), // TODO: config::APPLICATION_NAME
                icon: "".to_string(),       // TODO
                samples: ggez::conf::NumSamples::Zero,
            },
            backend: ggez::conf::Backend::default(),
            modules: ggez::conf::ModuleConf {
                gamepad: true,
                audio: true,
            },
        }
    }
}
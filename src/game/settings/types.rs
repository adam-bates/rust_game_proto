use super::GameResult;
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
    window_width: usize,
    window_height: usize,
    window_mode: FullscreenType,
    aspect_ratio: AspectRatio,
    // colour_blind_mode,
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            window_width: 1920,  // TODO
            window_height: 1080, // TODO
            window_mode: FullscreenType::Windowed,
            aspect_ratio: AspectRatio::Stretch,
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

pub fn build_default_settings() -> Settings {
    Settings::default() // TODO
}

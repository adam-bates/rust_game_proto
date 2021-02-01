use super::{config, input::types::GameButton, GameResult};
use ggez::{conf::FullscreenType, input::keyboard::KeyCode};
use serde_derive::{Deserialize, Serialize};
use serde_with::serde_as;
use std::{collections::HashMap, io};
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

    #[serde(skip)]
    pub inverse_target_fps: f32,

    #[serde(skip)]
    pub inverse_target_fps_duration: std::time::Duration,
    // colour_blind_mode,
}

impl VideoSettings {
    fn apply(&mut self, ctx: &mut ggez::Context) -> GameResult {
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

        {
            self.inverse_target_fps = 1. / self.target_fps as f32;
            self.inverse_target_fps_duration =
                std::time::Duration::from_secs_f32(self.inverse_target_fps);
        }

        Ok(())
    }
}

const DEFAULT_FPS_U32: u32 = 60;
const DEFAULT_INVERSE_FPS_F32: f32 = 1. / DEFAULT_FPS_U32 as f32;

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            windowed_width: config::VIEWPORT_PIXELS_WIDTH_USIZE,
            windowed_height: config::VIEWPORT_PIXELS_HEIGHT_USIZE,
            fullscreen_type: FullscreenType::Windowed,
            aspect_ratio: AspectRatio::Stretch,
            target_fps: DEFAULT_FPS_U32,
            inverse_target_fps: DEFAULT_INVERSE_FPS_F32,
            inverse_target_fps_duration: std::time::Duration::from_secs_f32(
                DEFAULT_INVERSE_FPS_F32,
            ),
        }
    }
}

fn build_default_controller_button_mappings() -> HashMap<gilrs::Button, GameButton> {
    let mut mappings = HashMap::new();
    mappings.insert(gilrs::Button::DPadUp, GameButton::Up);
    mappings.insert(gilrs::Button::DPadDown, GameButton::Down);
    mappings.insert(gilrs::Button::DPadLeft, GameButton::Left);
    mappings.insert(gilrs::Button::DPadRight, GameButton::Right);
    mappings.insert(gilrs::Button::Start, GameButton::Start);
    mappings.insert(gilrs::Button::Select, GameButton::Select);
    mappings.insert(gilrs::Button::Mode, GameButton::Start);
    mappings.insert(gilrs::Button::North, GameButton::Secondary);
    mappings.insert(gilrs::Button::East, GameButton::Secondary);
    mappings.insert(gilrs::Button::South, GameButton::Primary);
    mappings.insert(gilrs::Button::West, GameButton::Primary);
    mappings
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControllerAxisMappings {
    pub controller_x_axis: gilrs::Axis,
    pub controller_y_axis: gilrs::Axis,
    pub invert_x: bool,
    pub invert_y: bool,
}

impl Default for ControllerAxisMappings {
    fn default() -> Self {
        Self {
            controller_x_axis: gilrs::Axis::LeftStickX,
            controller_y_axis: gilrs::Axis::LeftStickY,
            invert_x: false,
            invert_y: false,
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct ControllerSettings {
    #[serde_as(as = "Vec<(_, _)>")]
    pub controller_button_mappings: HashMap<gilrs::Button, GameButton>,

    pub controller_stick_deadzone: f32,
    pub controller_axis_mappings: ControllerAxisMappings,
}

impl Default for ControllerSettings {
    fn default() -> Self {
        Self {
            controller_stick_deadzone: 0.5,
            controller_button_mappings: build_default_controller_button_mappings(),
            controller_axis_mappings: ControllerAxisMappings::default(),
        }
    }
}

fn build_default_keyboard_mappings() -> HashMap<KeyCode, GameButton> {
    let mut mappings = HashMap::new();
    mappings.insert(KeyCode::Up, GameButton::Up);
    mappings.insert(KeyCode::Down, GameButton::Down);
    mappings.insert(KeyCode::Left, GameButton::Left);
    mappings.insert(KeyCode::Right, GameButton::Right);
    mappings.insert(KeyCode::W, GameButton::Up);
    mappings.insert(KeyCode::S, GameButton::Down);
    mappings.insert(KeyCode::A, GameButton::Left);
    mappings.insert(KeyCode::D, GameButton::Right);
    mappings.insert(KeyCode::Return, GameButton::Primary);
    mappings.insert(KeyCode::LShift, GameButton::Secondary);
    mappings.insert(KeyCode::RShift, GameButton::Secondary);
    mappings.insert(KeyCode::Escape, GameButton::Start);
    mappings.insert(KeyCode::Delete, GameButton::Select);
    mappings.insert(KeyCode::Back, GameButton::Select);
    mappings
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyboardSettings {
    #[serde_as(as = "Vec<(_, _)>")]
    pub keyboard_mappings: HashMap<KeyCode, GameButton>,
}

impl Default for KeyboardSettings {
    fn default() -> Self {
        Self {
            keyboard_mappings: build_default_keyboard_mappings(),
        }
    }
}

// locale, font, text-speed, ui-border-type
#[derive(Debug, Serialize, Deserialize)]
pub struct GameSettings {
    #[serde(rename = "controller")]
    pub controller_settings: ControllerSettings,

    #[serde(rename = "keyboard")]
    pub keyboard_settings: KeyboardSettings,
    // locale,
    // font,
    // text_speed,
    // ui_border_type
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            controller_settings: ControllerSettings::default(),
            keyboard_settings: KeyboardSettings::default(),
        }
    }
}

impl GameSettings {
    fn apply(&self, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }
}

// Volumes, sound-accessibility
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AudioSettings {
    // ...
}

impl AudioSettings {
    fn apply(&self, _ctx: &mut ggez::Context) -> GameResult {
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
    pub fn apply(&mut self, ctx: &mut ggez::Context) -> GameResult {
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

        Ok(toml::from_str(&encoded)?)
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
impl Into<ggez::conf::WindowMode> for &mut Settings {
    fn into(self) -> ggez::conf::WindowMode {
        (&*self).into()
    }
}
impl Into<ggez::conf::WindowMode> for Settings {
    fn into(self) -> ggez::conf::WindowMode {
        (&self).into()
    }
}

impl Into<ggez::conf::WindowSetup> for &Settings {
    fn into(self) -> ggez::conf::WindowSetup {
        ggez::conf::WindowSetup {
            vsync: true,
            srgb: true,
            title: config::APPLICATION_NAME.to_string(),
            icon: config::APPLICATION_ICON_FILE_PATH.to_string(),
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

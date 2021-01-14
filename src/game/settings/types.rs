// Resolution, window-mode (fullscreen, windowed, windowed-fullscreen), aspect ratio, color-blind mode
#[derive(Default)]
pub struct VideoSettings;

// Controls (input mapping), locale, font, text-speed, ui-border-type
#[derive(Default)]
pub struct GameSettings;

// Volumes, sound-accessibility
#[derive(Default)]
pub struct AudioSettings;

#[derive(Default)]
pub struct Settings {
    pub video_settings: VideoSettings,
    pub game_settings: GameSettings,
    pub audio_settings: AudioSettings,
}

pub fn build_default_settings() -> Settings {
    Settings::default() // TODO
}

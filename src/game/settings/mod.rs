// Resolution, window-mode (fullscreen, windowed, windowed-fullscreen), aspect ratio, color-blind mode
pub struct VideoSettings;

// Controls (input mapping), locale, font, text-speed, ui-border-type
pub struct GameSettings;

// Volumes, sound-accessibility
pub struct AudioSettings;

pub struct Settings {
    pub video_settings: VideoSettings,
    pub game_settings: GameSettings,
    pub audio_settings: AudioSettings,
}

pub fn find_or_default_for_user() -> Settings {
    Settings {
        video_settings: VideoSettings,
        game_settings: GameSettings,
        audio_settings: AudioSettings,
    }
}

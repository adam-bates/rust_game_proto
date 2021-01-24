use super::settings::Settings;
use ggez::input::keyboard::KeyCode;
use gilrs::Button;
use serde_derive::{Deserialize, Serialize};

const NEG_3_PI_BY_4: f32 = -3. * std::f32::consts::FRAC_PI_4;
const NEG_PI_BY_4: f32 = -1. * std::f32::consts::FRAC_PI_4;
const POS_PI_BY_4: f32 = std::f32::consts::FRAC_PI_4;
const POS_3_PI_BY_4: f32 = 3. * std::f32::consts::FRAC_PI_4;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameDirection {
    Up,
    Down,
    Left,
    Right,
}

impl GameDirection {
    const UP: (isize, isize) = (0, -1);
    const DOWN: (isize, isize) = (0, 1);
    const LEFT: (isize, isize) = (-1, 0);
    const RIGHT: (isize, isize) = (1, 0);

    pub fn to_xy(&self) -> (isize, isize) {
        match self {
            Self::Up => Self::UP,
            Self::Down => Self::DOWN,
            Self::Left => Self::LEFT,
            Self::Right => Self::RIGHT,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum GameButton {
    Up,
    Down,
    Left,
    Right,
    Primary,
    Secondary,
    Start,
    Select, // TODO: Should we use X/Y buttons? Or L/R buttons?
}

#[derive(Debug, Clone)]
pub enum GameInput {
    Direction { direction: Option<GameDirection> },
    Button { button: GameButton, pressed: bool },
}

impl GameInput {
    pub fn from_keycode(keycode: &KeyCode, pressed: bool, settings: &Settings) -> Option<Self> {
        settings
            .game_settings
            .keyboard_settings
            .keyboard_mappings
            .get(keycode)
            .cloned()
            .map(|button| Self::Button { button, pressed })
    }

    pub fn from_gamepad_button(
        button: &Button,
        pressed: bool,
        settings: &Settings,
    ) -> Option<Self> {
        settings
            .game_settings
            .controller_settings
            .controller_button_mappings
            .get(button)
            .cloned()
            .map(|button| Self::Button { button, pressed })
    }

    pub fn from_gamepad_axes(axis_x: f32, axis_y: f32, deadzone: f32) -> Self {
        if axis_x.hypot(axis_y) < deadzone {
            return Self::Direction { direction: None };
        }

        let angle = axis_x.atan2(axis_y);

        // Left: -3PI/4 to -PI/4
        // Up: -PI/4 to PI/4
        // Right: PI/4 to 3PI/4
        // Down: else
        let direction = if NEG_3_PI_BY_4 < angle && angle <= NEG_PI_BY_4 {
            GameDirection::Left
        } else if NEG_PI_BY_4 < angle && angle <= POS_PI_BY_4 {
            GameDirection::Up
        } else if POS_PI_BY_4 < angle && angle <= POS_3_PI_BY_4 {
            GameDirection::Right
        } else {
            GameDirection::Down
        };

        return Self::Direction {
            direction: Some(direction),
        };
    }
}

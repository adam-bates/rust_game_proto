use ggez::input::keyboard::KeyCode;
use gilrs::Button;

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
    const UP: (i32, i32) = (0, -1);
    const DOWN: (i32, i32) = (0, 1);
    const LEFT: (i32, i32) = (-1, 0);
    const RIGHT: (i32, i32) = (1, 0);

    pub fn to_xy(&self) -> (i32, i32) {
        match self {
            Self::Up => Self::UP,
            Self::Down => Self::DOWN,
            Self::Left => Self::LEFT,
            Self::Right => Self::RIGHT,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    pub fn from_keycode(keycode: &KeyCode, pressed: bool) -> Option<Self> {
        match *keycode {
            KeyCode::W | KeyCode::Up => Some(Self::Button {
                button: GameButton::Up,
                pressed,
            }),
            KeyCode::S | KeyCode::Down => Some(Self::Button {
                button: GameButton::Down,
                pressed,
            }),
            KeyCode::A | KeyCode::Left => Some(Self::Button {
                button: GameButton::Left,
                pressed,
            }),
            KeyCode::D | KeyCode::Right => Some(Self::Button {
                button: GameButton::Right,
                pressed,
            }),
            KeyCode::Return => Some(Self::Button {
                button: GameButton::Primary,
                pressed,
            }),
            KeyCode::LShift | KeyCode::RShift => Some(Self::Button {
                button: GameButton::Secondary,
                pressed,
            }),
            KeyCode::Escape => Some(Self::Button {
                button: GameButton::Start,
                pressed,
            }),
            KeyCode::Delete | KeyCode::Back => Some(Self::Button {
                button: GameButton::Select,
                pressed,
            }),
            _ => None,
        }
    }

    pub fn from_gamepad_button(button: &Button, pressed: bool) -> Option<Self> {
        None
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

    pub fn to_game_button(self) -> Option<GameButton> {
        match self {
            GameInput::Button { button, .. } => Some(button),
            _ => None,
        }
    }

    pub fn to_game_direction(self) -> Option<Option<GameDirection>> {
        match self {
            GameInput::Direction { direction } => Some(direction),
            _ => None,
        }
    }
}

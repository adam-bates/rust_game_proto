use ggez::input::keyboard::KeyCode;

#[derive(Debug, PartialEq, Eq)]
pub enum GameDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameButton {
    Primary,
    Secondary,
    Start,
    Select, // TODO: Should we use X/Y buttons? Or L/R buttons?
}

#[derive(Debug)]
pub enum GameInput {
    Direction { direction: Option<GameDirection> },
    Button { button: GameButton, pressed: bool },
}

impl GameInput {
    pub fn from_keycode(keycode: &KeyCode, pressed: bool) -> Option<Self> {
        match *keycode {
            KeyCode::W | KeyCode::Up => Some(Self::Direction {
                direction: Some(GameDirection::Up),
            }),
            KeyCode::S | KeyCode::Down => Some(Self::Direction {
                direction: Some(GameDirection::Down),
            }),
            KeyCode::A | KeyCode::Left => Some(Self::Direction {
                direction: Some(GameDirection::Left),
            }),
            KeyCode::D | KeyCode::Right => Some(Self::Direction {
                direction: Some(GameDirection::Right),
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

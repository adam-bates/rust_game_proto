use ggez::input::keyboard::KeyCode;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub enum GameInput {
    Up { pressed: bool },
    Down { pressed: bool },
    Left { pressed: bool },
    Right { pressed: bool },
    Primary { pressed: bool },
    Secondary { pressed: bool },
    Start { pressed: bool },
    Select { pressed: bool }, // TODO: Should we use X/Y buttons? Or L/R buttons?
}

impl GameInput {
    pub fn from_keycode(keycode: &KeyCode, pressed: bool) -> Option<Self> {
        match *keycode {
            KeyCode::W | KeyCode::Up => Some(Self::Up { pressed }),
            KeyCode::S | KeyCode::Down => Some(Self::Down { pressed }),
            KeyCode::A | KeyCode::Left => Some(Self::Left { pressed }),
            KeyCode::D | KeyCode::Right => Some(Self::Right { pressed }),
            KeyCode::Return => Some(Self::Primary { pressed }),
            KeyCode::LShift | KeyCode::RShift => Some(Self::Secondary { pressed }),
            KeyCode::Escape => Some(Self::Start { pressed }),
            KeyCode::Delete | KeyCode::Back => Some(Self::Select { pressed }),
            _ => None,
        }
    }

    pub fn to_direction(&self) -> Option<Direction> {
        match self {
            GameInput::Up { .. } => Some(Direction::Up),
            GameInput::Down { .. } => Some(Direction::Down),
            GameInput::Left { .. } => Some(Direction::Left),
            GameInput::Right { .. } => Some(Direction::Right),
            _ => None,
        }
    }
}

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
    Up,
    Down,
    Left,
    Right,
    Primary,
    Secondary,
    Start,
    Select, // TODO: Should we use X/Y buttons? Or L/R buttons?
}

impl GameInput {
    pub fn from_keycode(keycode: &KeyCode) -> Option<Self> {
        match *keycode {
            KeyCode::W | KeyCode::Up => Some(Self::Up),
            KeyCode::S | KeyCode::Down => Some(Self::Down),
            KeyCode::A | KeyCode::Left => Some(Self::Left),
            KeyCode::D | KeyCode::Right => Some(Self::Right),
            KeyCode::Return => Some(Self::Primary),
            KeyCode::LShift | KeyCode::RShift => Some(Self::Secondary),
            KeyCode::Escape => Some(Self::Start),
            KeyCode::Delete | KeyCode::Back => Some(Self::Select),
            _ => None,
        }
    }

    pub fn to_direction(&self) -> Option<Direction> {
        match self {
            GameInput::Up => Some(Direction::Up),
            GameInput::Down => Some(Direction::Down),
            GameInput::Left => Some(Direction::Left),
            GameInput::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

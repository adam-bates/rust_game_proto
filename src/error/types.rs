pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type GameResult<T = ()> = ggez::GameResult<T>;

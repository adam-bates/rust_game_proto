use std::{error, result};

pub type Result<T = ()> = result::Result<T, Box<dyn error::Error>>;
pub type IOResult<T = ()> = result::Result<T, std::io::Error>;
pub type LogSetupResult<T = ()> = std::result::Result<T, log::SetLoggerError>;
pub type GameResult<T = ()> = ggez::GameResult<T>;

use super::types;
use log::error;

// TODO: Actually try to handle error
pub fn handle_log_setup_err(err: log::SetLoggerError) -> types::Result {
    Err(Box::new(err))
}

// TODO: Actually try to handle error, or properly log
pub fn handle_game_err(err: ggez::GameError) -> types::Result {
    error!("handle_game_err: {}", err);
    Err(Box::new(err))
}

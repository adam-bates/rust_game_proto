use super::types;
use log::error;

// TODO: Actually try to handle error, or properly log
pub fn handle_game_err(err: ggez::GameError) -> types::Result {
    error!("handle_game_err: {}", err);
    Err(Box::new(err))
}

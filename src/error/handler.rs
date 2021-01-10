use log::error;

// TODO: Actually try to handle error, or properly log
pub fn handle_game_err(err: ggez::GameError) -> std::result::Result<(), Box<dyn std::error::Error>> {
  error!("handle_game_err: {}", err);
  Err(Box::new(err))
}

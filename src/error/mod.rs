pub mod types;

mod handler;

pub fn handle_game_err(err: ggez::GameError) -> types::Result {
  handler::handle_game_err(err)
}

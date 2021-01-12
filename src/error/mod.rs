mod handler;

pub mod types;

pub fn handle_game_err(err: ggez::GameError) -> types::Result {
    handler::handle_game_err(err)
}

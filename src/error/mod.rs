mod handler;

pub mod types;

pub fn handle_log_setup_err(err: log::SetLoggerError) -> types::Result {
    handler::handle_log_setup_err(err)
}
pub fn handle_game_err(err: ggez::GameError) -> types::Result {
    handler::handle_game_err(err)
}

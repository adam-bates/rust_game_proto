mod game;

use super::config;
use super::error;

pub fn run_game(
    ctx: &mut ggez::Context,
    events_loop: &mut ggez::event::EventsLoop,
) -> ggez::GameResult {
    game::run_game(ctx, events_loop)
}

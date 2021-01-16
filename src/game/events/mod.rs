mod gilrs_events;
mod winit_events;

use super::{error::types::GameResult, game_state};
use winit::Event;

pub fn process_event(
    ctx: &mut ggez::Context,
    state: &mut game_state::MainState,
    event: Event,
) -> GameResult {
    winit_events::process_event(ctx, state, event)
}

pub fn process_gamepad(ctx: &mut ggez::Context, state: &mut game_state::MainState) -> GameResult {
    gilrs_events::process_gamepad(ctx, state)
}

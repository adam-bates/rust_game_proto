use super::{error::types::GameResult, events, game_state};
use ggez::event::EventHandler;

// Main game loop
pub fn run(
    ctx: &mut ggez::Context,
    events_loop: &mut ggez::event::EventsLoop,
    mut state: game_state::MainState,
) -> GameResult {
    while ctx.continuing {
        ctx.timer_context.tick();

        events_loop.poll_events(|event| {
            // Don't need this call as it unnecessarily clones the event,
            // So I moved all the logic into our event processing
            // ctx.process_event(&event);

            if let Err(_) = events::process_event(ctx, &mut state, event) {
                // TODO: Handle game error
            }
        });

        events::process_gamepad(ctx, &mut state)?;

        state.update(ctx)?;
        state.draw(ctx)?;
    }

    Ok(())
}

use super::{
    error::types::GameResult,
    events::{self, EventHandler},
    game_state,
};

const EPSILON_DURATION: std::time::Duration = std::time::Duration::from_nanos(1);

// Main update run
fn run_update(
    ctx: &mut ggez::Context,
    state: &mut game_state::MainState,
    changed: bool,
) -> GameResult<bool> {
    let mut update_changed = changed;

    while ggez::timer::check_update_time(ctx, state.target_fps) {
        update_changed = true;
        state.update(ctx)?;
    }

    if !changed {
        // Give CPU room to breathe
        ggez::timer::yield_now();
        std::thread::sleep(EPSILON_DURATION);
        core::sync::atomic::spin_loop_hint();
    }

    Ok(update_changed)
}

// Main draw run
fn run_draw(
    ctx: &mut ggez::Context,
    state: &game_state::MainState,
    changed: bool,
) -> GameResult<bool> {
    let mut draw_changed = changed;

    // Only update context if game-state has changed
    if draw_changed {
        draw_changed = false;
        // Clear Window
        ggez::graphics::clear(ctx, ggez::graphics::BLACK);

        state.draw(ctx)?;
    } else {
        // Give CPU room to breathe
        std::thread::yield_now();
        std::thread::sleep(EPSILON_DURATION);
        core::sync::atomic::spin_loop_hint();
    }

    ggez::graphics::present(ctx)?;

    Ok(draw_changed)
}

// Main game loop
pub fn run(
    ctx: &mut ggez::Context,
    events_loop: &mut ggez::event::EventsLoop,
    mut state: game_state::MainState,
) -> GameResult {
    let mut changed = false;

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

        changed = run_update(ctx, &mut state, changed)?;
        changed = run_draw(ctx, &state, changed)?;

        ggez::graphics::present(ctx)?
    }

    Ok(())
}

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

    while ggez::timer::check_update_time(ctx, state.settings.video_settings.target_fps) {
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

        // Let render target call draw on state
        state.render_target.draw(state, ctx)?;
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
    mut ctx: ggez::Context,
    event_loop: ggez::event::EventLoop<()>,
    mut state: game_state::MainState,
) -> GameResult {
    let mut state_changed = false;

    ggez::graphics::set_default_filter(&mut ctx, ggez::graphics::FilterMode::Nearest);

    while ctx.continuing {
        ctx.timer_context.tick();

        event_loop.run(move |event, _, control_flow| {
            if !ctx.continuing {
                *control_flow = winit::event_loop::ControlFlow::Exit;
                return;
            }

            *control_flow = winit::event_loop::ControlFlow::Poll;

            let ctx = &mut ctx;
            let state = &mut state;

            // Don't need this call as it unnecessarily clones the event,
            // So I moved all the logic into our event processing...
            // ctx.process_event(&event);

            if let Err(_) = events::process_event(ctx, state, event, &mut state_changed) {
                // TODO: Handle game error
            }
        });
    }

    Ok(())
}

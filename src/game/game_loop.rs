use super::{events, game_state};

// Main game loop
pub fn run(
    mut ctx: ggez::Context,
    event_loop: ggez::event::EventLoop<()>,
    mut state: game_state::GlobalState,
    error_handler: Box<dyn Fn(ggez::GameError)>,
) {
    let mut state_changed = false;
    event_loop.run(move |event, _, control_flow| {
        let ctx = &mut ctx;
        let state = &mut state;

        if let Err(e) = events::process_event(ctx, state, event, &mut state_changed) {
            *control_flow = winit::event_loop::ControlFlow::Exit;
            error_handler(e);
            return;
        }

        if !ctx.continuing {
            *control_flow = winit::event_loop::ControlFlow::Exit;
            return;
        }

        *control_flow = winit::event_loop::ControlFlow::Poll;
    });
}

use super::{events, game_state};

use tracing_flame::FlameLayer;
use tracing_subscriber::{fmt, prelude::*};

fn setup_global_subscriber() -> impl Drop {
    let fmt_layer = fmt::layer()
        .with_ansi(true)
        .with_target(true)
        .with_thread_names(true)
        .with_thread_ids(false);

    let (flame_layer, _guard) = FlameLayer::with_file("./tracing/tracing.folded").unwrap();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(flame_layer)
        .init();

    _guard
}

// Main game loop
pub fn run(
    mut ctx: ggez::Context,
    event_loop: ggez::event::EventLoop<()>,
    mut state: game_state::GlobalState,
    error_handler: Box<dyn Fn(ggez::GameError)>,
) {
    let _ = setup_global_subscriber();
    event_loop.run(move |event, _, control_flow| {
        tracing::info_span!("main loop run").in_scope(|| {
            let ctx = &mut ctx;
            let state = &mut state;

            if let Err(e) = events::process_event(ctx, state, event) {
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
    });
}

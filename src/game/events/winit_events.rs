use super::{config, game_state, EventHandler, GameResult};
use ggez::{
    event::{
        quit,
        winit_event::{
            DeviceEvent, ElementState, Event, KeyboardInput, MouseScrollDelta, WindowEvent,
        },
    },
    input::{keyboard, mouse},
};
use winit::dpi;

fn process_window_event(
    ctx: &mut ggez::Context,
    state: &mut game_state::MainState,
    event: WindowEvent,
) -> GameResult {
    match event {
        WindowEvent::Resized(physical_size) => {
            // From ctx.process_event(&event)
            {
                ctx.gfx_context.window.resize(physical_size);
                ctx.gfx_context.resize_viewport();
            }

            state.resize_event(ctx, physical_size.width as f32, physical_size.height as f32)?;
        }
        WindowEvent::CloseRequested => {
            quit(ctx);
        }
        WindowEvent::Focused(gained) => {
            state.focus_event(ctx, gained)?;
        }
        WindowEvent::ReceivedCharacter(ch) => {
            state.text_input_event(ctx, ch)?;
        }
        WindowEvent::ModifiersChanged(mods) => ctx
            .keyboard_context
            .set_modifiers(keyboard::KeyMods::from(mods)),
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state: element_state,
                    virtual_keycode: Some(keycode),
                    ..
                },
            ..
        } => match element_state {
            ggez::event::winit_event::ElementState::Pressed => {
                // From ctx.process_event(&event)
                {
                    ctx.keyboard_context.set_key(keycode, true);
                }

                if !keyboard::is_key_repeated(ctx) {
                    state.key_down_event(ctx, keycode)?;
                }
            }
            ggez::event::winit_event::ElementState::Released => {
                // From ctx.process_event(&event)
                {
                    ctx.keyboard_context.set_key(keycode, false);
                }

                state.key_up_event(ctx, keycode)?;
            }
        },
        WindowEvent::MouseWheel { delta, .. } => {
            let (x, y) = match delta {
                MouseScrollDelta::LineDelta(x, y) => (x, y),
                MouseScrollDelta::PixelDelta(dpi::PhysicalPosition { x, y }) => {
                    (x as f32, y as f32)
                }
            };
            state.mouse_wheel_event(ctx, x, y)?;
        }
        WindowEvent::MouseInput {
            state: element_state,
            button,
            ..
        } => {
            // From ctx.process_event(&event)
            {
                let pressed = match element_state {
                    ggez::event::winit_event::ElementState::Pressed => true,
                    ggez::event::winit_event::ElementState::Released => false,
                };
                ctx.mouse_context.set_button(button, pressed);
            }

            let position = mouse::position(ctx);
            let coord_x = config::VIEWPORT_TILES_WIDTH_F32 * position.x / state.window_coords.w;
            let coord_y = config::VIEWPORT_TILES_HEIGHT_F32 * position.y / state.window_coords.h;

            match element_state {
                ElementState::Pressed => {
                    state.mouse_button_down_event(ctx, button, coord_x, coord_y)?;
                }
                ElementState::Released => {
                    state.mouse_button_up_event(ctx, button, coord_x, coord_y)?;
                }
            }
        }
        WindowEvent::CursorMoved {
            position: physical_position,
            ..
        } => {
            // From ctx.process_event(&event)
            {
                ctx.mouse_context
                    .set_last_position(ggez::graphics::Point2::new(
                        physical_position.x as f32,
                        physical_position.y as f32,
                    ));
            }

            let position = mouse::position(ctx);
            let delta = mouse::delta(ctx);
            state.mouse_motion_event(ctx, position.x, position.y, delta.x, delta.y)?;
        }
        _ => {}
    }

    Ok(())
}

fn process_device_event(
    ctx: &mut ggez::Context,
    _state: &mut game_state::MainState,
    event: DeviceEvent,
) -> GameResult {
    match event {
        DeviceEvent::MouseMotion { delta: (x, y) } => {
            ctx.mouse_context
                .set_last_delta(ggez::graphics::Point2::new(x as f32, y as f32));
        }
        _ => {}
    }

    Ok(())
}

const EPSILON_DURATION: std::time::Duration = std::time::Duration::from_nanos(1);

// Main update run
fn run_update(
    ctx: &mut ggez::Context,
    state: &mut game_state::MainState,
    state_changed: bool,
) -> GameResult<bool> {
    let mut update_changed = state_changed;

    while ggez::timer::check_update_time(ctx, state.settings.video_settings.target_fps) {
        update_changed = true;
        state.update(ctx)?;
    }

    if !state_changed {
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
    state_changed: bool,
) -> GameResult<bool> {
    let mut draw_changed = state_changed;

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

pub fn process_event(
    ctx: &mut ggez::Context,
    state: &mut game_state::MainState,
    event: Event<()>,
    state_changed: &mut bool,
) -> GameResult {
    match event {
        Event::WindowEvent { event, .. } => process_window_event(ctx, state, event)?,
        Event::DeviceEvent { event, .. } => process_device_event(ctx, state, event)?,
        Event::MainEventsCleared => {
            ctx.timer_context.tick();

            super::process_gamepad(ctx, state)?;

            *state_changed = run_update(ctx, state, *state_changed)?;
            *state_changed = run_draw(ctx, &state, *state_changed)?;
        }
        _ => {}
    }

    Ok(())
}

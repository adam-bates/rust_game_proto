use super::{error::types::GameResult, game_state};
use ggez::{
    event::{quit, EventHandler},
    input::{gamepad::GamepadId, keyboard, mouse},
};
use winit::{dpi, DeviceEvent, ElementState, Event, KeyboardInput, MouseScrollDelta, WindowEvent};

fn process_window_event(
    ctx: &mut ggez::Context,
    state: &mut game_state::MainState,
    event: WindowEvent,
) -> GameResult {
    match event {
        WindowEvent::Resized(logical_size) => {
            // From ctx.process_event(&event)
            {
                let hidpi_factor = ctx.gfx_context.window.get_hidpi_factor();
                let physical_size = logical_size.to_physical(hidpi_factor as f64);
                ctx.gfx_context.window.resize(physical_size);
                ctx.gfx_context.resize_viewport();
            }

            // let actual_size = logical_size;
            state.resize_event(ctx, logical_size.width as f32, logical_size.height as f32);
        }
        WindowEvent::CloseRequested => {
            if !state.quit_event(ctx) {
                quit(ctx);
            }
        }
        WindowEvent::Focused(gained) => {
            state.focus_event(ctx, gained);
        }
        WindowEvent::ReceivedCharacter(ch) => {
            state.text_input_event(ctx, ch);
        }
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state: element_state,
                    virtual_keycode: Some(keycode),
                    modifiers,
                    ..
                },
            ..
        } => match element_state {
            ggez::event::winit_event::ElementState::Pressed => {
                // From ctx.process_event(&event)
                {
                    ctx.keyboard_context
                        .set_modifiers(keyboard::KeyMods::from(modifiers));
                    ctx.keyboard_context.set_key(keycode, true);
                }

                let repeat = keyboard::is_key_repeated(ctx);
                state.key_down_event(ctx, keycode, modifiers.into(), repeat);
            }
            ggez::event::winit_event::ElementState::Released => {
                // From ctx.process_event(&event)
                {
                    ctx.keyboard_context
                        .set_modifiers(keyboard::KeyMods::from(modifiers));
                    ctx.keyboard_context.set_key(keycode, false);
                }
                state.key_up_event(ctx, keycode, modifiers.into());
            }
        },
        WindowEvent::MouseWheel { delta, .. } => {
            let (x, y) = match delta {
                MouseScrollDelta::LineDelta(x, y) => (x, y),
                MouseScrollDelta::PixelDelta(dpi::LogicalPosition { x, y }) => (x as f32, y as f32),
            };
            state.mouse_wheel_event(ctx, x, y);
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
            match element_state {
                ElementState::Pressed => {
                    state.mouse_button_down_event(ctx, button, position.x, position.y)
                }
                ElementState::Released => {
                    state.mouse_button_up_event(ctx, button, position.x, position.y)
                }
            }
        }
        WindowEvent::CursorMoved {
            position: logical_position,
            ..
        } => {
            // From ctx.process_event(&event)
            {
                ctx.mouse_context
                    .set_last_position(ggez::graphics::Point2::new(
                        logical_position.x as f32,
                        logical_position.y as f32,
                    ));
            }

            let position = mouse::position(ctx);
            let delta = mouse::delta(ctx);
            state.mouse_motion_event(ctx, position.x, position.y, delta.x, delta.y);
        }
        WindowEvent::Moved(_) => {}
        WindowEvent::Destroyed => {}
        WindowEvent::DroppedFile(_) => {}
        WindowEvent::HoveredFile(_) => {}
        WindowEvent::HoveredFileCancelled => {}
        WindowEvent::CursorEntered { device_id } => {}
        WindowEvent::CursorLeft { device_id } => {}
        WindowEvent::TouchpadPressure {
            device_id,
            pressure,
            stage,
        } => {}
        WindowEvent::AxisMotion {
            device_id,
            axis,
            value,
        } => {}
        WindowEvent::Refresh => {}
        WindowEvent::Touch(_) => {}
        WindowEvent::HiDpiFactorChanged(_) => {}
        _x => {
            // trace!("ignoring window event {:?}", x);
        }
    }

    Ok(())
}

fn process_device_event(
    ctx: &mut ggez::Context,
    state: &mut game_state::MainState,
    event: DeviceEvent,
) -> GameResult {
    if let ggez::event::winit_event::DeviceEvent::MouseMotion { delta: (x, y) } = event {
        ctx.mouse_context
            .set_last_delta(ggez::graphics::Point2::new(x as f32, y as f32));
    }

    match event {
        DeviceEvent::Added => {}
        DeviceEvent::Removed => {}
        DeviceEvent::MouseMotion { delta } => {}
        DeviceEvent::MouseWheel { delta } => {}
        DeviceEvent::Motion { axis, value } => {}
        DeviceEvent::Button { button, state } => {}
        DeviceEvent::Key(_) => {}
        DeviceEvent::Text { codepoint } => {}
    }

    Ok(())
}

fn process_gamepad(ctx: &mut ggez::Context, state: &mut game_state::MainState) -> GameResult {
    while let Some(gilrs::Event { id, event, .. }) = ctx.gamepad_context.next_event() {
        match event {
            gilrs::EventType::ButtonPressed(button, _) => {
                state.gamepad_button_down_event(ctx, button, GamepadId(id));
            }
            gilrs::EventType::ButtonReleased(button, _) => {
                state.gamepad_button_up_event(ctx, button, GamepadId(id));
            }
            gilrs::EventType::AxisChanged(axis, value, _) => {
                state.gamepad_axis_event(ctx, axis, value, GamepadId(id));
            }
            gilrs::EventType::ButtonRepeated(_, _) => {}
            gilrs::EventType::ButtonChanged(_, _, _) => {}
            gilrs::EventType::Connected => {}
            gilrs::EventType::Disconnected => {}
            gilrs::EventType::Dropped => {}
        }
    }

    Ok(())
}

fn process_event(
    ctx: &mut ggez::Context,
    state: &mut game_state::MainState,
    event: Event,
) -> GameResult {
    match event {
        Event::WindowEvent { event, .. } => process_window_event(ctx, state, event)?,
        Event::DeviceEvent { event, .. } => process_device_event(ctx, state, event)?,
        Event::Awakened => {}
        Event::Suspended(_) => {}
    }

    Ok(())
}

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

            if let Err(_) = process_event(ctx, &mut state, event) {
                // TODO: Handle game error
            }
        });

        process_gamepad(ctx, &mut state)?;

        state.update(ctx)?;
        state.draw(ctx)?;
    }

    Ok(())
}

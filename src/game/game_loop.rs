use super::{error::types::GameResult, game_state};
use ggez::{
    event::{quit, EventHandler},
    input::{gamepad::GamepadId, keyboard, mouse},
};
use winit::{dpi, DeviceEvent, ElementState, Event, KeyboardInput, MouseScrollDelta, WindowEvent};

pub fn run(ctx: &mut ggez::Context, events_loop: &mut ggez::event::EventsLoop) -> GameResult {
    let mut state = game_state::MainState {};

    while ctx.continuing {
        // If you are writing your own event loop, make sure
        // you include `timer_context.tick()` and
        // `ctx.process_event()` calls.  These update ggez's
        // internal state however necessary.
        ctx.timer_context.tick();
        events_loop.poll_events(|event| {
            ctx.process_event(&event);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(logical_size) => {
                        // let actual_size = logical_size;
                        state.resize_event(
                            ctx,
                            logical_size.width as f32,
                            logical_size.height as f32,
                        );
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
                                state: ElementState::Pressed,
                                virtual_keycode: Some(keycode),
                                modifiers,
                                ..
                            },
                        ..
                    } => {
                        let repeat = keyboard::is_key_repeated(ctx);
                        state.key_down_event(ctx, keycode, modifiers.into(), repeat);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Released,
                                virtual_keycode: Some(keycode),
                                modifiers,
                                ..
                            },
                        ..
                    } => {
                        state.key_up_event(ctx, keycode, modifiers.into());
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        let (x, y) = match delta {
                            MouseScrollDelta::LineDelta(x, y) => (x, y),
                            MouseScrollDelta::PixelDelta(dpi::LogicalPosition { x, y }) => {
                                (x as f32, y as f32)
                            }
                        };
                        state.mouse_wheel_event(ctx, x, y);
                    }
                    WindowEvent::MouseInput {
                        state: element_state,
                        button,
                        ..
                    } => {
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
                    WindowEvent::CursorMoved { .. } => {
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
                },
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::Added => {}
                    DeviceEvent::Removed => {}
                    DeviceEvent::MouseMotion { delta } => {}
                    DeviceEvent::MouseWheel { delta } => {}
                    DeviceEvent::Motion { axis, value } => {}
                    DeviceEvent::Button { button, state } => {}
                    DeviceEvent::Key(_) => {}
                    DeviceEvent::Text { codepoint } => {}
                },
                Event::Awakened => (),
                Event::Suspended(_) => (),
            }
        });
        // Handle gamepad events if necessary.
        if ctx.conf.modules.gamepad {
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
        }
        state.update(ctx)?;
        state.draw(ctx)?;
    }

    Ok(())
}

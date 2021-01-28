use super::{game_state, EventHandler, GameResult};
use ggez::input::gamepad::GamepadId;

#[tracing::instrument]
pub fn process_gamepad(ctx: &mut ggez::Context, state: &mut game_state::GlobalState) -> GameResult {
    while let Some(gilrs::Event { id, event, .. }) = ctx.gamepad_context.next_event() {
        match event {
            gilrs::EventType::ButtonPressed(button, _) => {
                state.gamepad_button_down_event(ctx, button, GamepadId(id))?;
            }
            gilrs::EventType::ButtonReleased(button, _) => {
                state.gamepad_button_up_event(ctx, button, GamepadId(id))?;
            }
            gilrs::EventType::AxisChanged(axis, value, _) => {
                state.gamepad_axis_event(ctx, axis, value, GamepadId(id))?;
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

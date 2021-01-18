mod gilrs_events;
mod winit_events;

use super::{config, error::types::GameResult, game_state};
use ggez::{
    event::winit_event::{Event, MouseButton},
    input::{gamepad::GamepadId, keyboard::KeyCode},
    Context,
};
use gilrs::ev::{Axis, Button};

pub trait EventHandler {
    fn update(&mut self, ctx: &mut Context) -> GameResult;
    fn draw(&self, ctx: &mut Context) -> GameResult;

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> GameResult {
        Ok(())
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) -> GameResult {
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, _keycode: KeyCode) -> GameResult {
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode) -> GameResult {
        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) -> GameResult {
        Ok(())
    }

    fn gamepad_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _btn: Button,
        _id: GamepadId,
    ) -> GameResult {
        Ok(())
    }

    fn gamepad_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _btn: Button,
        _id: GamepadId,
    ) -> GameResult {
        Ok(())
    }

    fn gamepad_axis_event(
        &mut self,
        _ctx: &mut Context,
        _axis: Axis,
        _value: f32,
        _id: GamepadId,
    ) -> GameResult {
        Ok(())
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) -> GameResult {
        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) -> GameResult {
        Ok(())
    }
}

pub fn process_event(
    ctx: &mut ggez::Context,
    state: &mut game_state::MainState,
    event: Event<()>,
    state_changed: &mut bool,
) -> GameResult {
    winit_events::process_event(ctx, state, event, state_changed)
}

pub fn process_gamepad(ctx: &mut ggez::Context, state: &mut game_state::MainState) -> GameResult {
    gilrs_events::process_gamepad(ctx, state)
}

use super::{error::types::GameResult, events, settings};

pub struct MainState;

impl MainState {
    pub fn new(ctx: &mut ggez::Context, user_settings: settings::Settings) -> Self {
        Self
    }
}

impl events::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn draw(&self, ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }
}

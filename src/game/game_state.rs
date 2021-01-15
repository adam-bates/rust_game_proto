use super::settings;

pub struct MainState;

impl MainState {
    pub fn new(ctx: &mut ggez::Context, user_settings: settings::Settings) -> Self {
        Self
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
}

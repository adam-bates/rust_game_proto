use super::error::types;
use super::context;
use super::settings;

pub fn run_game(mut fs: ggez::filesystem::Filesystem) -> types::GameResult {
    fs.log_all();
    
    let user_settings = settings::find_or_default_for_user();
    let (ctx, events_loop) = &mut context::new_context(fs, &user_settings);
    
    // ...

    // TODO: Customize
    ggez::event::run(ctx, events_loop, &mut State {})
}

struct State;
impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
}

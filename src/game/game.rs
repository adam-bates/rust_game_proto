use super::error::types;
use super::context;
use super::settings;

pub fn run_game(mut fs: ggez::filesystem::Filesystem) -> types::GameResult {
    let user_settings = settings::find_or_default_for_user(&mut fs)?;
    let (ctx, events_loop) = &mut context::new_context(fs, &user_settings);

    println!("{:?}", user_settings);
    
    // ...

    // TODO: Customize
    let mut state = State {};
    ggez::event::run(ctx, events_loop, &mut state)
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

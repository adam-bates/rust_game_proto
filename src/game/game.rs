use super::context;
use super::error::types;
use super::settings;

pub fn run_game(mut fs: ggez::filesystem::Filesystem) -> types::GameResult {
    let (mut user_settings, first_load) = settings::find_or_default_for_user(&mut fs)?;
    let (ctx, events_loop) = &mut context::new_context(fs, &user_settings)?;

    if first_load {
        settings::initialize_first_load(ctx, &mut user_settings)?;
    }

    ctx.filesystem.print_all();
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

use super::context;
use super::error::types;
use super::game_loop;
use super::game_state;
use super::settings;

pub fn run_game(mut fs: ggez::filesystem::Filesystem) -> types::GameResult {
    let (mut user_settings, first_load) = settings::find_or_default_for_user(&mut fs)?;
    let (mut ctx, events_loop) = context::new_context(fs, &user_settings)?;

    if first_load {
        settings::initialize_first_load(&mut ctx, &mut user_settings)?;
    }

    let state = game_state::MainState::new(&mut ctx, user_settings)?;

    let _w = specs::World::default();

    game_loop::run(ctx, events_loop, state)
}

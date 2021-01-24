use super::{
    ecs::resources::DeltaTime,
    error::types::GameResult,
    game_state::GameState,
    input::types::GameInput,
    types::{Scene, SceneSwitch},
};

pub struct InGameScene;

impl InGameScene {
    pub fn new(game_state: &mut GameState, _ctx: &mut ggez::Context) -> GameResult<Self> {
        // TODO: Build from loaded save file

        game_state.world.insert(DeltaTime::default());

        Ok(Self)
    }

    // TODO: Function to build from save file given a filesystem
}

impl Scene for InGameScene {
    fn dispose(&mut self, game_state: &mut GameState, _ctx: &mut ggez::Context) -> GameResult {
        game_state.world.remove::<DeltaTime>();
        Ok(())
    }

    fn update(
        &mut self,
        game_state: &mut GameState,
        _ctx: &mut ggez::Context,
        delta_secs: f32,
    ) -> GameResult<Option<SceneSwitch>> {
        if let Some(mut delta) = game_state.world.get_mut::<DeltaTime>() {
            delta.secs = delta_secs;
        }

        Ok(None)
    }

    fn draw(&self, _game_state: &GameState, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn input(
        &mut self,
        _game_state: &mut GameState,
        _ctx: &mut ggez::Context,
        input: GameInput,
    ) -> GameResult<Option<SceneSwitch>> {
        println!("{:?}", input);
        Ok(None)
    }
}

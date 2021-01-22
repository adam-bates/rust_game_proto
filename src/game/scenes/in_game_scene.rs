use super::{
    ecs::resources::DeltaTime,
    error::types::GameResult,
    game_state::GameState,
    input::types::GameInput,
    types::{Scene, SceneSwitch},
};

pub struct InGameScene {}

impl InGameScene {
    pub fn new(game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        // TODO: Build from loaded save file

        Ok(Self {})
    }

    // TODO: Function to build from save file given a filesystem
}

impl Scene for InGameScene {
    fn update(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>> {
        if let Some(mut delta) = game_state.world.get_mut::<DeltaTime>() {
            delta.duration = ggez::timer::delta(ctx);
        }

        Ok(None)
    }

    fn draw(&self, game_state: &GameState, ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn input(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        input: GameInput,
    ) -> GameResult<Option<SceneSwitch>> {
        Ok(None)
    }
}

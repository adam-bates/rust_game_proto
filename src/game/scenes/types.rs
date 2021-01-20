use super::{error::types::GameResult, game_state::GameState, input::GameInput};

pub enum SceneSwitch {
    Pop,
    Push(Box<dyn Scene>),
    Replace(Box<dyn Scene>),
}

// Structs that impl Scene will hold state that is "local" to that scene
pub trait Scene {
    fn update(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>>;

    fn draw(
        &self,
        game_state: &GameState,
        ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>>;

    fn input(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        input: GameInput,
    ) -> GameResult<Option<SceneSwitch>>;

    fn should_draw_previous(&self) -> bool {
        false
    }
}

pub struct SceneManager {
    //
}

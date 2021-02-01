use super::{
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameButton, GameInput},
    types::{Scene, SceneSwitch},
};

pub struct PauseMenuScene;

impl PauseMenuScene {
    pub fn new() -> Self {
        println!("Paused");

        PauseMenuScene
    }
}

impl std::fmt::Debug for PauseMenuScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {{ ... }}", self.name()))
    }
}

impl Scene for PauseMenuScene {
    fn dispose(&mut self, _game_state: &mut GameState, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn update(
        &mut self,
        _game_state: &mut GameState,
        _ctx: &mut ggez::Context,
        _delta_secs: f32,
    ) -> GameResult<Option<SceneSwitch>> {
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
        match input {
            GameInput::Button { button, pressed } => {
                if pressed {
                    match button {
                        GameButton::Start | GameButton::Secondary => {
                            return Ok(Some(SceneSwitch::Pop))
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };

        Ok(None)
    }

    fn should_draw_previous(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        "PauseMenuScene"
    }
}

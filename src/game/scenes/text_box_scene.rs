use super::{
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameButton, GameInput},
    types::{Scene, SceneSwitch},
};

pub struct TextBoxScene;

impl TextBoxScene {
    pub fn new(text: &str) -> Self {
        println!("Text Box: {}", text);

        // TODO: Presentation:
        // - Show text box with text
        // - Allow for "multi-page" text boxes depending on length of text
        // - Maybe dim main screen somehow (shader?), or take in Options
        // - Allow user input to answer questions?
        // - Handle user's locale using "Fluent"

        TextBoxScene
    }
}

impl std::fmt::Debug for TextBoxScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {{ ... }}", self.name()))
    }
}

impl Scene for TextBoxScene {
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
                        GameButton::Primary => return Ok(Some(SceneSwitch::Pop)),
                        _ => {}
                    }
                }
            }
            _ => {}
        };

        Ok(None)
    }

    fn should_update_previous(&self) -> bool {
        true
    }

    fn should_draw_previous(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        "TextBoxScene"
    }
}

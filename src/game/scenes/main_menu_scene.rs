use super::{
    config,
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameButton, GameInput},
    types::{Scene, SceneSwitch},
};

pub struct MainMenuScene {
    mesh: ggez::graphics::Mesh,
    mesh_param: ggez::graphics::DrawParam,
    rotation: f32,
}

impl MainMenuScene {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        let mesh = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(0., 0., 5., 50.),
            ggez::graphics::Color::from_rgb(255, 50, 50),
        )?;
        let mesh_param = ggez::graphics::DrawParam::default().dest([
            config::VIEWPORT_PIXELS_WIDTH_F32 / 2.,
            config::VIEWPORT_PIXELS_HEIGHT_F32 / 2.,
        ]);

        Ok(Self {
            mesh,
            mesh_param,
            rotation: 0f32.atan2(0f32),
        })
    }
}

impl Scene for MainMenuScene {
    fn dispose(&mut self, game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn update(
        &mut self,
        _game_state: &mut GameState,
        _ctx: &mut ggez::Context,
        delta_secs: f32,
    ) -> GameResult<Option<SceneSwitch>> {
        Ok(None)
    }

    fn draw(&self, _game_state: &GameState, ctx: &mut ggez::Context) -> GameResult {
        ggez::graphics::clear(ctx, ggez::graphics::WHITE);
        ggez::graphics::draw(ctx, &self.mesh, self.mesh_param.rotation(self.rotation))?;

        Ok(())
    }

    fn input(
        &mut self,
        _game_state: &mut GameState,
        ctx: &mut ggez::Context,
        input: GameInput,
    ) -> GameResult<Option<SceneSwitch>> {
        if ggez::input::keyboard::is_mod_active(ctx, ggez::input::keyboard::KeyMods::CTRL) {
            match input {
                GameInput::Button { button, .. } => match button {
                    GameButton::Select => return Ok(Some(SceneSwitch::Pop)),
                    _ => {}
                },
                // ggez::event::KeyCode::S => self.settings.save(),
                _ => {}
            }
        }
        Ok(None)
    }
}

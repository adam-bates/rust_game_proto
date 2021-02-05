use super::{
    config,
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameButton, GameInput},
    save::SaveSlot,
    settings,
    types::{Scene, SceneBuilder, SceneSwitch},
    InGameScene,
};
use ggez::graphics::Drawable as GgezDrawable;
use std::{cell::RefCell, rc::Rc};

pub struct MainMenuScene {
    save_slot: usize,
    background_color: ggez::graphics::Color,
    text: ggez::graphics::Text,
    text_param: ggez::graphics::DrawParam,
}

impl MainMenuScene {
    pub fn new(_game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        let resolution = settings::get_current_monitor_resolution(ctx)?;

        let monitor_scale_width = resolution.0 / config::VIEWPORT_PIXELS_WIDTH_F32;
        let monitor_scale_height = resolution.1 / config::VIEWPORT_PIXELS_HEIGHT_F32;

        // Render text at monitor resolution for smooth resizing
        let text_scale = monitor_scale_width.max(monitor_scale_height);

        let font = ggez::graphics::Font::new(ctx, "/fonts/DejaVuSansMono.ttf")?;
        let text = ggez::graphics::Text::new(
            ggez::graphics::TextFragment::new("Press Start")
                .font(font)
                .scale(ggez::graphics::PxScale::from(15. * text_scale))
                .color(ggez::graphics::Color::from_rgb(50, 0, 200)),
        );

        let text_pos_x =
            (text_scale * config::VIEWPORT_PIXELS_WIDTH_F32 - text.width(ctx) as f32) / 2.;
        let text_pos_y =
            (text_scale * config::VIEWPORT_PIXELS_HEIGHT_F32 - text.height(ctx) as f32) / 2.;

        Ok(Self {
            save_slot: 1,
            background_color: ggez::graphics::Color::from_rgb(112, 200, 160),
            text,
            text_param: ggez::graphics::DrawParam::default()
                .dest([text_pos_x / text_scale, text_pos_y / text_scale])
                .scale([1. / text_scale, 1. / text_scale]),
        })
    }
}

impl std::fmt::Debug for MainMenuScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {{ ... }}", self.name()))
    }
}

impl Scene for MainMenuScene {
    fn dispose(&mut self, _game_state: &mut GameState, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    #[tracing::instrument]
    fn update(
        &mut self,
        _game_state: &mut GameState,
        _ctx: &mut ggez::Context,
        _delta_secs: f32,
    ) -> GameResult<Option<SceneSwitch>> {
        Ok(None)
    }

    #[tracing::instrument]
    fn draw(&self, _game_state: &GameState, ctx: &mut ggez::Context) -> GameResult {
        ggez::graphics::clear(ctx, self.background_color);

        self.text.draw(ctx, self.text_param)?;

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
                        GameButton::Primary | GameButton::Start => {
                            match SaveSlot::from_id(self.save_slot) {
                                Some(save_slot) => {
                                    println!("Starting save slot: {}", save_slot.id());
                                    let scene_builder: SceneBuilder =
                                        Box::new(move |game_state, ctx| {
                                            let scene =
                                                InGameScene::new(game_state, ctx, save_slot)?;

                                            Ok(Rc::new(RefCell::new(scene)))
                                        });

                                    return Ok(Some(SceneSwitch::ReplaceAll(scene_builder)));
                                }
                                _ => println!("Invalid save slot: {}", self.save_slot),
                            }
                        }
                        GameButton::Right => {
                            self.save_slot += 1;
                            println!("Save slot: {}", self.save_slot);
                        }
                        GameButton::Left => {
                            self.save_slot = (self.save_slot as isize - 1).max(1) as usize;
                            println!("Save slot: {}", self.save_slot);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn name(&self) -> &str {
        "MainMenuScene"
    }
}

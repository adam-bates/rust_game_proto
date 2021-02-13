use super::{
    config,
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameButton, GameInput},
    save::{self, MetaSaveData, SaveSlot},
    settings,
    types::{Scene, SceneBuilder, SceneSwitch},
    InGameScene,
};
use ggez::graphics::Drawable as GgezDrawable;
use std::{cell::RefCell, rc::Rc};

pub struct MainMenuScene {
    background_color: ggez::graphics::Color,
    text: ggez::graphics::Text,
    text_param: ggez::graphics::DrawParam,
    saves: Vec<Option<MetaSaveData>>,
    selected_save: Option<usize>,
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

        let mut saves = vec![];
        for save_slot in SaveSlot::all() {
            saves.push(save::load_meta(ctx, save_slot)?);
        }

        Ok(Self {
            background_color: ggez::graphics::Color::from_rgb(112, 200, 160),
            text,
            text_param: ggez::graphics::DrawParam::default()
                .dest([text_pos_x / text_scale, text_pos_y / text_scale])
                .scale([1. / text_scale, 1. / text_scale]),
            selected_save: None,
            saves,
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
                            if let Some(selected_save) = self.selected_save {
                                match SaveSlot::from_id(selected_save) {
                                    Some(save_slot) => {
                                        let opt_meta_data_save = self.saves[selected_save].clone();

                                        println!("Starting save slot: {}", save_slot.id());
                                        let scene_builder: SceneBuilder = Box::new(
                                            move |game_state, ctx| {
                                                let meta_data = match &opt_meta_data_save {
                                                    Some(meta_data) => meta_data.clone(),
                                                    None => {
                                                        save::new_save(
                                                            ctx,
                                                            save_slot,
                                                            format!("Adam{}", save_slot.id()),
                                                        )?;
                                                        save::load_meta(ctx, save_slot)?.expect(
                                                            &format!("Couldn't load meta save data after creating new save: {:?}", save_slot)
                                                        )
                                                    }
                                                };

                                                let scene = InGameScene::new(
                                                    game_state, ctx, save_slot, meta_data,
                                                )?;

                                                Ok(Rc::new(RefCell::new(scene)))
                                            },
                                        );

                                        return Ok(Some(SceneSwitch::ReplaceAll(scene_builder)));
                                    }
                                    _ => println!("Invalid save slot: {}", selected_save),
                                }
                            }
                        }
                        GameButton::Up => {
                            self.selected_save = Some(0);
                            println!("Save slot: {:?}", self.selected_save);
                            if let Some(selected_save) = self.selected_save {
                                println!("Save: {:?}", self.saves[selected_save]);
                            }
                        }
                        GameButton::Down => {
                            self.selected_save = None;
                            println!("Save slot: {:?}", self.selected_save);
                            if let Some(selected_save) = self.selected_save {
                                println!("Save: {:?}", self.saves[selected_save]);
                            }
                        }
                        GameButton::Right => {
                            if let Some(selected_save) = &mut self.selected_save {
                                *selected_save = (*selected_save + 1) % self.saves.len();
                            }
                            println!("Save slot: {:?}", self.selected_save);
                            if let Some(selected_save) = self.selected_save {
                                println!("Save: {:?}", self.saves[selected_save]);
                            }
                        }
                        GameButton::Left => {
                            if let Some(selected_save) = &mut self.selected_save {
                                if *selected_save == 0 {
                                    *selected_save = self.saves.len() - 1;
                                } else {
                                    *selected_save -= 1;
                                }
                            }
                            println!("Save slot: {:?}", self.selected_save);
                            if let Some(selected_save) = self.selected_save {
                                println!("Save: {:?}", self.saves[selected_save]);
                            }
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

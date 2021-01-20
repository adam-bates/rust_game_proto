use super::{
    config,
    error::types::GameResult,
    events,
    input::types::GameInput,
    render::state::RenderState,
    scenes::types::SceneManager,
    settings::{AspectRatio, Settings},
};

#[derive(Default)]
pub struct InputState {
    gamepad_axis_x: f32,
    gamepad_axis_y: f32,
}

pub struct GameState {
    pub input_state: InputState,
    pub render_state: RenderState,
    pub settings: Settings,
}

impl GameState {
    fn new(ctx: &mut ggez::Context, settings: Settings) -> GameResult<Self> {
        Ok(Self {
            input_state: InputState::default(),
            render_state: RenderState::new(ctx, &settings)?,
            settings,
        })
    }
}

pub struct GlobalState {
    pub scene_manager: SceneManager,
    pub game_state: GameState,
}

impl GlobalState {
    pub fn new(ctx: &mut ggez::Context, settings: Settings) -> GameResult<Self> {
        Ok(Self {
            scene_manager: SceneManager::new(ctx, &settings)?,
            game_state: GameState::new(ctx, settings)?,
        })
    }

    // TODO: Remove
    fn toggle_fullscreen(&mut self, ctx: &mut ggez::Context) -> GameResult {
        self.game_state.settings.video_settings.fullscreen_type =
            match self.game_state.settings.video_settings.fullscreen_type {
                ggez::conf::FullscreenType::Windowed => ggez::conf::FullscreenType::Desktop,
                ggez::conf::FullscreenType::Desktop => ggez::conf::FullscreenType::True,
                ggez::conf::FullscreenType::True => ggez::conf::FullscreenType::Windowed,
            };

        self.game_state.settings.apply(ctx)?;

        Ok(())
    }
}

impl events::EventHandler for GlobalState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // TODO: Update game using scene manager
        Ok(())
    }

    fn draw(&self, ctx: &mut ggez::Context) -> GameResult {
        // TODO: Draw game using scene manager

        ggez::graphics::clear(ctx, ggez::graphics::WHITE);
        let mesh = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(0., 0., 5., 50.),
            ggez::graphics::Color::from_rgb(255, 50, 50),
        )?;

        let x = -1. * self.game_state.input_state.gamepad_axis_x;
        let y = -1. * self.game_state.input_state.gamepad_axis_y;

        ggez::graphics::draw(
            ctx,
            &mesh,
            ggez::graphics::DrawParam::default()
                .dest([
                    config::VIEWPORT_PIXELS_WIDTH_F32 / 2.,
                    config::VIEWPORT_PIXELS_HEIGHT_F32 / 2.,
                ])
                .rotation(x.atan2(y)),
        )?;
        // let image = ggez::graphics::Image::new(ctx, "/background_pallet_town.png")?;
        // let background_width = 24.;
        // let background_height = 20.;

        // let mut sprite_batch = ggez::graphics::spritebatch::SpriteBatch::new(image);

        // let inverse_background_width = 1. / background_width;
        // let inverse_background_height = 1. / background_height;

        // let camera_width = config::VIEWPORT_TILES_WIDTH_USIZE as i32;
        // let camera_height = config::VIEWPORT_TILES_HEIGHT_USIZE as i32;

        // let pos_x = background_width as i32 - camera_width;
        // let pos_y = background_height as i32 - camera_height;

        // for x in pos_x..camera_width + pos_x {
        //     for y in pos_y..camera_height + pos_y {
        //         sprite_batch.add(
        //             ggez::graphics::DrawParam::default()
        //                 .src(
        //                     [
        //                         x as f32 * inverse_background_width,
        //                         y as f32 * inverse_background_height,
        //                         inverse_background_width,
        //                         inverse_background_height,
        //                     ]
        //                     .into(),
        //                 )
        //                 .dest([
        //                     (x - pos_x) as f32 * config::TILE_PIXELS_SIZE_F32,
        //                     (y - pos_y) as f32 * config::TILE_PIXELS_SIZE_F32,
        //                 ]),
        //         );
        //     }
        // }

        // use ggez::graphics::Drawable;
        // sprite_batch.draw(ctx, ggez::graphics::DrawParam::default())?;

        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut ggez::Context, width: f32, height: f32) -> GameResult {
        if self.game_state.settings.video_settings.fullscreen_type
            == ggez::conf::FullscreenType::Windowed
        {
            self.game_state.settings.video_settings.windowed_width = width as usize;
            self.game_state.settings.video_settings.windowed_height = height as usize;
        }

        self.game_state.render_state.window_coords =
            ggez::graphics::Rect::new(0.0, 0.0, width, height);

        self.game_state
            .render_state
            .refresh(ctx, &self.game_state.settings.video_settings.aspect_ratio)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: ggez::input::keyboard::KeyCode,
    ) -> GameResult {
        if ggez::input::keyboard::is_mod_active(ctx, ggez::input::keyboard::KeyMods::CTRL) {
            match keycode {
                ggez::input::keyboard::KeyCode::Q => ggez::event::quit(ctx),
                ggez::input::keyboard::KeyCode::F => self.toggle_fullscreen(ctx)?,
                ggez::input::keyboard::KeyCode::A => {
                    let aspect_ratio = match self.game_state.settings.video_settings.aspect_ratio {
                        AspectRatio::Ratio16By9 => AspectRatio::Ratio4By3,
                        AspectRatio::Ratio4By3 => AspectRatio::PixelPerfect,
                        AspectRatio::PixelPerfect => AspectRatio::Stretch,
                        AspectRatio::Stretch => AspectRatio::Ratio16By9,
                    };
                    println!("Set aspect ratio: {:?}", aspect_ratio);

                    self.game_state.settings.video_settings.aspect_ratio = aspect_ratio;
                    self.game_state
                        .render_state
                        .refresh(ctx, &self.game_state.settings.video_settings.aspect_ratio)?;
                }
                // ggez::event::KeyCode::S => self.settings.save(),
                _ => {}
            }
        } else if let Some(game_input) = GameInput::from_keycode(&keycode, true) {
            println!("{:?}", game_input);

            if let Some(direction) = game_input.to_game_direction() {
                // self.game_state.input_state.requested_direction = direction;
            }
        }

        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: ggez::input::keyboard::KeyCode,
    ) -> GameResult {
        if let Some(game_input) = GameInput::from_keycode(&keycode, false) {
            println!("{:?}", game_input);
            if let Some(direction) = game_input.to_game_direction() {
                // if self.game_state.input_state.requested_direction == direction {
                //     self.game_state.input_state.requested_direction = None;
                // }
            }
        }

        Ok(())
    }

    fn gamepad_axis_event(
        &mut self,
        _ctx: &mut ggez::Context,
        axis: gilrs::Axis,
        value: f32,
        _id: ggez::input::gamepad::GamepadId,
    ) -> GameResult {
        match axis {
            gilrs::ev::Axis::LeftStickX => {
                self.game_state.input_state.gamepad_axis_x = value;
            }
            gilrs::ev::Axis::LeftStickY => {
                self.game_state.input_state.gamepad_axis_y = value;
            }
            _ => return Ok(()),
        }

        let gamepad_axis_x = self.game_state.input_state.gamepad_axis_x;
        let gamepad_axis_y = self.game_state.input_state.gamepad_axis_y;
        let controller_stick_deadzone = self
            .game_state
            .settings
            .game_settings
            .controller_stick_deadzone;

        let game_input =
            GameInput::from_gamepad_axes(gamepad_axis_x, gamepad_axis_y, controller_stick_deadzone);

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: ggez::event::winit_event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        println!("Mouse Button Down: {:?} @ [{}, {}]", button, x, y);
        Ok(())
    }
}

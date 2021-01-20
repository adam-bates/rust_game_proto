use super::{
    config,
    error::types::GameResult,
    events, input,
    render::target::{CanvasRenderTarget, RenderTarget, WindowRenderTarget},
    scenes::types::SceneManager,
    settings::{AspectRatio, Settings},
};

#[derive(Default)]
pub struct InputState {
    requested_direction: Option<input::GameDirection>,
    axis_vector: (f32, f32),
}

pub struct RenderState {
    pub render_target: Box<dyn RenderTarget>,
    pub screen_coords: ggez::graphics::Rect,
    pub window_coords: ggez::graphics::Rect,
    pub window_color_format: ggez::graphics::Format,
}

pub struct GameState {
    pub input_state: InputState,
    pub render_state: RenderState,
    pub settings: Settings,
}

pub struct GlobalState {
    pub scene_manager: SceneManager,
    pub game_state: GameState,
}

impl GlobalState {
    pub fn new(ctx: &mut ggez::Context, settings: Settings) -> GameResult<Self> {
        let render_target = Box::new(WindowRenderTarget);

        let screen_coords = ggez::graphics::Rect::new(
            0.,
            0.,
            config::VIEWPORT_PIXELS_WIDTH_F32,
            config::VIEWPORT_PIXELS_HEIGHT_F32,
        );

        ggez::graphics::set_screen_coordinates(ctx, screen_coords)?;

        let window_coords = ggez::graphics::Rect::new(
            0.,
            0.,
            settings.video_settings.windowed_width as f32,
            settings.video_settings.windowed_height as f32,
        );

        let render_state = RenderState {
            render_target,
            screen_coords,
            window_coords,
            window_color_format: ggez::graphics::get_window_color_format(ctx),
        };

        let game_state = GameState {
            input_state: InputState::default(),
            render_state,
            settings,
        };

        Ok(Self {
            scene_manager: SceneManager {}, // TODO
            game_state,
        })
    }

    pub fn update_render_target(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let (canvas_width, canvas_height) =
            match self.game_state.settings.video_settings.aspect_ratio {
                AspectRatio::Ratio16By9 => {
                    const RATIO_16_9: f32 = 16. / 9.;
                    const RATIO_9_16: f32 = 9. / 16.;

                    if self.game_state.render_state.window_coords.w
                        / self.game_state.render_state.window_coords.h
                        < RATIO_16_9
                    {
                        (
                            self.game_state.render_state.window_coords.w,
                            self.game_state.render_state.window_coords.w * RATIO_9_16,
                        )
                    } else {
                        (
                            self.game_state.render_state.window_coords.h * RATIO_16_9,
                            self.game_state.render_state.window_coords.h,
                        )
                    }
                }
                AspectRatio::Ratio4By3 => {
                    const RATIO_4_3: f32 = 4. / 3.;
                    const RATIO_3_4: f32 = 3. / 4.;

                    if self.game_state.render_state.window_coords.w
                        / self.game_state.render_state.window_coords.h
                        < RATIO_4_3
                    {
                        (
                            self.game_state.render_state.window_coords.w,
                            self.game_state.render_state.window_coords.w * RATIO_3_4,
                        )
                    } else {
                        (
                            self.game_state.render_state.window_coords.h * RATIO_4_3,
                            self.game_state.render_state.window_coords.h,
                        )
                    }
                }
                AspectRatio::PixelPerfect => {
                    const RATIO_W_H: f32 =
                        config::VIEWPORT_TILES_WIDTH_F32 / config::VIEWPORT_TILES_HEIGHT_F32;
                    const RATIO_H_W: f32 =
                        config::VIEWPORT_TILES_HEIGHT_F32 / config::VIEWPORT_TILES_WIDTH_F32;

                    if self.game_state.render_state.window_coords.w
                        / self.game_state.render_state.window_coords.h
                        < RATIO_W_H
                    {
                        (
                            self.game_state.render_state.window_coords.w,
                            self.game_state.render_state.window_coords.w * RATIO_H_W,
                        )
                    } else {
                        (
                            self.game_state.render_state.window_coords.h * RATIO_W_H,
                            self.game_state.render_state.window_coords.h,
                        )
                    }
                }
                // Don't render canvas if stretched aspect
                AspectRatio::Stretch => {
                    if !self.game_state.render_state.render_target.is_window() {
                        self.game_state.render_state.render_target = Box::new(WindowRenderTarget);
                        // Draw graphics at canvas resolution
                        ggez::graphics::set_screen_coordinates(
                            ctx,
                            self.game_state.render_state.screen_coords,
                        )?;
                    }

                    return Ok(());
                }
            };

        let canvas = ggez::graphics::Canvas::new(
            ctx,
            canvas_width as u16,
            canvas_height as u16,
            ggez::conf::NumSamples::One,
            self.game_state.render_state.window_color_format,
        )?;

        let canvas_param = ggez::graphics::DrawParam::default().dest([
            (self.game_state.render_state.window_coords.w - canvas_width) / 2.,
            (self.game_state.render_state.window_coords.h - canvas_height) / 2.,
        ]);

        self.game_state.render_state.render_target = Box::new(CanvasRenderTarget {
            canvas,
            canvas_param,
        });

        Ok(())
    }

    fn toggle_fullscreen(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let fullscreen_type = match self.game_state.settings.video_settings.fullscreen_type {
            ggez::conf::FullscreenType::Windowed => ggez::conf::FullscreenType::Desktop,
            ggez::conf::FullscreenType::Desktop => ggez::conf::FullscreenType::True,
            ggez::conf::FullscreenType::True => ggez::conf::FullscreenType::Windowed,
        };

        self.game_state.settings.video_settings.fullscreen_type = fullscreen_type;
        ggez::graphics::set_mode(ctx, (&self.game_state.settings).into())?;

        if fullscreen_type == ggez::conf::FullscreenType::Windowed {
            ggez::graphics::set_drawable_size(
                ctx,
                self.game_state.settings.video_settings.windowed_width as f32,
                self.game_state.settings.video_settings.windowed_height as f32,
            )?;
        }

        ggez::input::mouse::set_cursor_hidden(
            ctx,
            fullscreen_type == ggez::conf::FullscreenType::True,
        );

        Ok(())
    }
}

impl events::EventHandler for GlobalState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // TODO: Update game using scene manager
        if ggez::timer::ticks(ctx) % 100 == 0 {
            // println!("Direction: {:?}", self.input_state.requested_direction);
        }
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

        let x = -1. * self.game_state.input_state.axis_vector.0;
        let y = -1. * self.game_state.input_state.axis_vector.1;

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

        self.update_render_target(ctx)
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
                    self.game_state.settings.video_settings.aspect_ratio =
                        match self.game_state.settings.video_settings.aspect_ratio {
                            AspectRatio::Ratio16By9 => AspectRatio::Ratio4By3,
                            AspectRatio::Ratio4By3 => AspectRatio::PixelPerfect,
                            AspectRatio::PixelPerfect => AspectRatio::Stretch,
                            AspectRatio::Stretch => AspectRatio::Ratio16By9,
                        };
                    println!(
                        "Set aspect ratio: {:?}",
                        self.game_state.settings.video_settings.aspect_ratio
                    );
                    self.update_render_target(ctx)?;
                }
                // ggez::event::KeyCode::S => self.settings.save(),
                _ => {}
            }
        } else if let Some(game_input) = input::GameInput::from_keycode(&keycode, true) {
            println!("{:?}", game_input);

            if let Some(direction) = game_input.to_game_direction() {
                self.game_state.input_state.requested_direction = direction;
            }
        }

        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: ggez::input::keyboard::KeyCode,
    ) -> GameResult {
        if let Some(game_input) = input::GameInput::from_keycode(&keycode, false) {
            println!("{:?}", game_input);
            if let Some(direction) = game_input.to_game_direction() {
                if self.game_state.input_state.requested_direction == direction {
                    self.game_state.input_state.requested_direction = None;
                }
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
        const DEADZONE: f32 = 0.5; // TODO: settings

        match axis {
            gilrs::ev::Axis::LeftStickY => {
                self.game_state.input_state.axis_vector.1 = value;
            }
            gilrs::ev::Axis::LeftStickX => {
                self.game_state.input_state.axis_vector.0 = value;
            }
            _ => return Ok(()),
        }

        let length = self
            .game_state
            .input_state
            .axis_vector
            .0
            .hypot(self.game_state.input_state.axis_vector.1);

        if length < DEADZONE {
            self.game_state.input_state.requested_direction = None;
        } else {
            let angle = self
                .game_state
                .input_state
                .axis_vector
                .0
                .atan2(self.game_state.input_state.axis_vector.1);

            // Left: -3PI/4 to -PI/4
            // Up: -PI/4 to PI/4
            // Right: PI/4 to 3PI/4
            // Down: else

            const NEG_3_PI_BY_4: f32 = -3. * std::f32::consts::FRAC_PI_4;
            const NEG_PI_BY_4: f32 = -1. * std::f32::consts::FRAC_PI_4;
            const POS_PI_BY_4: f32 = std::f32::consts::FRAC_PI_4;
            const POS_3_PI_BY_4: f32 = 3. * std::f32::consts::FRAC_PI_4;

            let direction = if NEG_3_PI_BY_4 < angle && angle <= NEG_PI_BY_4 {
                input::GameDirection::Left
            } else if NEG_PI_BY_4 < angle && angle <= POS_PI_BY_4 {
                input::GameDirection::Up
            } else if POS_PI_BY_4 < angle && angle <= POS_3_PI_BY_4 {
                input::GameDirection::Right
            } else {
                input::GameDirection::Down
            };

            self.game_state.input_state.requested_direction = Some(direction);
        }

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

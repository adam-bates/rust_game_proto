use super::{
    config,
    error::types::GameResult,
    events,
    render::target::{CanvasRenderTarget, RenderTarget, WindowRenderTarget},
    settings::{AspectRatio, Settings},
};

pub struct MainState {
    pub settings: Settings,
    pub render_target: Box<dyn RenderTarget>,
    pub screen_coords: ggez::graphics::Rect,
    pub window_coords: ggez::graphics::Rect,
}

impl MainState {
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

        Ok(Self {
            settings,
            render_target,
            screen_coords,
            window_coords,
        })
    }

    pub fn update_render_target(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let (canvas_width, canvas_height) = match self.settings.video_settings.aspect_ratio {
            AspectRatio::Ratio16By9 => {
                const RATIO_16_9: f32 = 16. / 9.;
                const RATIO_9_16: f32 = 9. / 16.;

                if self.window_coords.w / self.window_coords.h < RATIO_16_9 {
                    (self.window_coords.w, self.window_coords.w * RATIO_9_16)
                } else {
                    (self.window_coords.h * RATIO_16_9, self.window_coords.h)
                }
            }
            AspectRatio::Ratio15By9 => {
                const RATIO_15_9: f32 = 15. / 9.;
                const RATIO_9_15: f32 = 9. / 15.;

                if self.window_coords.w / self.window_coords.h < RATIO_15_9 {
                    (self.window_coords.w, self.window_coords.w * RATIO_9_15)
                } else {
                    (self.window_coords.h * RATIO_15_9, self.window_coords.h)
                }
            }
            AspectRatio::Ratio4By3 => {
                const RATIO_4_3: f32 = 4. / 3.;
                const RATIO_3_4: f32 = 3. / 4.;

                if self.window_coords.w / self.window_coords.h < RATIO_4_3 {
                    (self.window_coords.w, self.window_coords.w * RATIO_3_4)
                } else {
                    (self.window_coords.h * RATIO_4_3, self.window_coords.h)
                }
            }
            // Don't render canvas if stretched aspect
            AspectRatio::Stretch => {
                if !self.render_target.is_window() {
                    self.render_target = Box::new(WindowRenderTarget);
                    // Draw graphics at canvas resolution
                    ggez::graphics::set_screen_coordinates(ctx, self.screen_coords)?;
                }

                return Ok(());
            }
        };

        let canvas = ggez::graphics::Canvas::new(
            ctx,
            canvas_width as u16,
            canvas_height as u16,
            ggez::conf::NumSamples::One,
        )?;

        let canvas_param = ggez::graphics::DrawParam::default().dest([
            (self.window_coords.w - canvas_width) / 2.,
            (self.window_coords.h - canvas_height) / 2.,
        ]);

        self.render_target = Box::new(CanvasRenderTarget {
            canvas,
            canvas_param,
        });

        Ok(())
    }

    fn toggle_fullscreen(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let fullscreen_type = match self.settings.video_settings.fullscreen_type {
            ggez::conf::FullscreenType::Windowed => ggez::conf::FullscreenType::Desktop,
            ggez::conf::FullscreenType::Desktop => ggez::conf::FullscreenType::True,
            ggez::conf::FullscreenType::True => ggez::conf::FullscreenType::Windowed,
        };

        self.settings.video_settings.fullscreen_type = fullscreen_type;
        ggez::graphics::set_mode(ctx, (&self.settings).into())?;

        if fullscreen_type == ggez::conf::FullscreenType::Windowed {
            ggez::graphics::set_drawable_size(
                ctx,
                self.settings.video_settings.windowed_width as f32,
                self.settings.video_settings.windowed_height as f32,
            )?;
        }

        ggez::input::mouse::set_cursor_hidden(
            ctx,
            fullscreen_type == ggez::conf::FullscreenType::True,
        );

        Ok(())
    }
}

impl events::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // TODO: Update game using scene manager
        Ok(())
    }

    fn draw(&self, ctx: &mut ggez::Context) -> GameResult {
        // TODO: Draw game using scene manager

        let image = ggez::graphics::Image::new(ctx, "/background_pallet_town.png")?;
        let mut sprite_batch = ggez::graphics::spritebatch::SpriteBatch::new(image);

        let tile_size = 16.;
        let background_width = 24.;
        let background_height = 20.;
        let inverse_background_width = 1. / background_width;
        let inverse_background_height = 1. / background_height;
        // let camera_width = 19;
        // let camera_height = 11;
        let camera_width = 16;
        let camera_height = 9;
        let pos_x = background_width as i32 - camera_width;
        let pos_y = background_height as i32 - camera_height;
        for x in pos_x..camera_width + pos_x {
            for y in pos_y..camera_height + pos_y {
                sprite_batch.add(
                    ggez::graphics::DrawParam::default()
                        .src(
                            [
                                x as f32 * inverse_background_width,
                                y as f32 * inverse_background_height,
                                inverse_background_width,
                                inverse_background_height,
                            ]
                            .into(),
                        )
                        // Stretch out the 16x16 tiles into whatever size/position gives a 15x9 aspect on the canvas
                        .scale([
                            config::VIEWPORT_PIXELS_WIDTH_F32 / (tile_size * camera_width as f32),
                            config::VIEWPORT_PIXELS_HEIGHT_F32 / (tile_size * camera_height as f32),
                        ])
                        .dest([
                            (x - pos_x) as f32 * config::VIEWPORT_PIXELS_WIDTH_F32
                                / camera_width as f32,
                            (y - pos_y) as f32 * config::VIEWPORT_PIXELS_HEIGHT_F32
                                / camera_height as f32,
                        ]),
                );
            }
        }

        use ggez::graphics::Drawable;
        sprite_batch.draw(ctx, ggez::graphics::DrawParam::default())?;

        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut ggez::Context, width: f32, height: f32) -> GameResult {
        if self.settings.video_settings.fullscreen_type == ggez::conf::FullscreenType::Windowed {
            self.settings.video_settings.windowed_width = width as usize;
            self.settings.video_settings.windowed_height = height as usize;
        }

        self.window_coords = ggez::graphics::Rect::new(0.0, 0.0, width, height);

        self.update_render_target(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: ggez::input::keyboard::KeyCode,
        _keymods: ggez::input::keyboard::KeyMods,
        repeat: bool,
    ) -> GameResult {
        if !repeat
            && ggez::input::keyboard::is_mod_active(ctx, ggez::input::keyboard::KeyMods::CTRL)
        {
            match keycode {
                ggez::input::keyboard::KeyCode::Q => ggez::event::quit(ctx),
                ggez::input::keyboard::KeyCode::F => self.toggle_fullscreen(ctx)?,
                ggez::input::keyboard::KeyCode::A => {
                    self.settings.video_settings.aspect_ratio =
                        match self.settings.video_settings.aspect_ratio {
                            AspectRatio::Ratio16By9 => AspectRatio::Ratio15By9,
                            AspectRatio::Ratio15By9 => AspectRatio::Ratio4By3,
                            AspectRatio::Ratio4By3 => AspectRatio::Stretch,
                            AspectRatio::Stretch => AspectRatio::Ratio16By9,
                        };
                    println!(
                        "Set aspect ratio: {:?}",
                        self.settings.video_settings.aspect_ratio
                    );
                    self.update_render_target(ctx)?;
                }
                // ggez::event::KeyCode::S => self.settings.save(),
                _ => {}
            }
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _button: winit::MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        println!("{:?}: [{}, {}]", _button, _x, _y);
        Ok(())
    }
}

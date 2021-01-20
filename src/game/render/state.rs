use super::{
    config,
    error::types::GameResult,
    settings::{AspectRatio, Settings},
    target::{CanvasRenderTarget, RenderTarget, WindowRenderTarget},
};

pub struct RenderState {
    pub render_target: Box<dyn RenderTarget>,
    pub screen_coords: ggez::graphics::Rect,
    pub window_coords: ggez::graphics::Rect,
    pub window_color_format: ggez::graphics::Format,
}

impl RenderState {
    pub fn new(ctx: &mut ggez::Context, settings: &Settings) -> GameResult<Self> {
        ggez::graphics::set_default_filter(ctx, ggez::graphics::FilterMode::Nearest);

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
            render_target,
            screen_coords,
            window_coords,
            window_color_format: ggez::graphics::get_window_color_format(ctx),
        })
    }

    pub fn refresh(&mut self, ctx: &mut ggez::Context, aspect_ratio: &AspectRatio) -> GameResult {
        let (canvas_width, canvas_height) = match aspect_ratio {
            AspectRatio::Ratio16By9 => {
                const RATIO_16_9: f32 = 16. / 9.;
                const RATIO_9_16: f32 = 9. / 16.;

                if self.window_coords.w / self.window_coords.h < RATIO_16_9 {
                    (self.window_coords.w, self.window_coords.w * RATIO_9_16)
                } else {
                    (self.window_coords.h * RATIO_16_9, self.window_coords.h)
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
            AspectRatio::PixelPerfect => {
                const RATIO_W_H: f32 =
                    config::VIEWPORT_TILES_WIDTH_F32 / config::VIEWPORT_TILES_HEIGHT_F32;
                const RATIO_H_W: f32 =
                    config::VIEWPORT_TILES_HEIGHT_F32 / config::VIEWPORT_TILES_WIDTH_F32;

                if self.window_coords.w / self.window_coords.h < RATIO_W_H {
                    (self.window_coords.w, self.window_coords.w * RATIO_H_W)
                } else {
                    (self.window_coords.h * RATIO_W_H, self.window_coords.h)
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
            self.window_color_format,
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
}

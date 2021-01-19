use super::{error::types::GameResult, events::EventHandler, game_state::MainState};
use ggez::{
    graphics::{self, Drawable},
    Context,
};

pub trait RenderTarget {
    fn draw(&self, state: &MainState, ctx: &mut Context) -> GameResult;
    fn is_window(&self) -> bool;
}

pub struct WindowRenderTarget;
impl RenderTarget for WindowRenderTarget {
    fn draw(&self, state: &MainState, ctx: &mut Context) -> GameResult {
        state.draw(ctx)
    }

    fn is_window(&self) -> bool {
        true
    }
}

pub struct CanvasRenderTarget {
    pub canvas: graphics::Canvas,
    pub canvas_param: graphics::DrawParam,
}
impl RenderTarget for CanvasRenderTarget {
    fn draw(&self, state: &MainState, ctx: &mut Context) -> GameResult {
        // Only need to clear when rendering to canvas to give us "black bars"
        graphics::clear(ctx, graphics::BLACK);

        // Set Canvas
        graphics::set_canvas(ctx, Some(&self.canvas));

        // Draw graphics at canvas resolution
        graphics::set_screen_coordinates(ctx, state.render_state.screen_coords)?;

        state.draw(ctx)?;

        // Start drawing to window
        graphics::set_canvas(ctx, None);
        graphics::set_screen_coordinates(ctx, state.render_state.window_coords)?;

        // Draw canvas onto window
        self.canvas.draw(ctx, self.canvas_param)?;

        Ok(())
    }

    fn is_window(&self) -> bool {
        false
    }
}

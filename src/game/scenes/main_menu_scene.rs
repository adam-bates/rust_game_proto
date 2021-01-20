use super::{
    config,
    error::types::GameResult,
    game_state::GameState,
    types::{Scene, SceneSwitch},
};

pub struct MainMenuScene {}

impl Scene for MainMenuScene {
    fn update(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>> {
        Ok(None)
    }

    fn draw(
        &self,
        game_state: &GameState,
        ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>> {
        ggez::graphics::clear(ctx, ggez::graphics::WHITE);
        let mesh = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(0., 0., 5., 50.),
            ggez::graphics::Color::from_rgb(255, 50, 50),
        )?;

        let x = -1. * game_state.input_state.gamepad_axis_x;
        let y = -1. * game_state.input_state.gamepad_axis_y;

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

        Ok(None)
    }

    fn input(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        input: crate::game::input::types::GameInput,
    ) -> GameResult<Option<SceneSwitch>> {
        Ok(None)
    }
}

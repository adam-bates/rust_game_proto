use super::{
    config,
    resources::{Camera, ShouldUpdateBackgroundTiles, TileMap},
};

#[derive(Debug)]
pub struct UpdateBackgroundTilesSystem;

impl<'a> specs::System<'a> for UpdateBackgroundTilesSystem {
    type SystemData = (
        specs::Write<'a, ShouldUpdateBackgroundTiles>,
        specs::Read<'a, Camera>,
        Option<specs::Write<'a, TileMap>>,
    );

    #[tracing::instrument(
        skip(should_update_background_tiles_r, camera_r, tile_map_r),
        name = "UpdateBackgroundTilesSystem"
    )]
    fn run(
        &mut self,
        (mut should_update_background_tiles_r, camera_r, tile_map_r): Self::SystemData,
    ) {
        if should_update_background_tiles_r.0 {
            should_update_background_tiles_r.0 = false;

            if let Some(mut tile_map) = tile_map_r {
                // Update background tiles to draw
                let (max_x, _) = tile_map.dimensions();

                tile_map.background.clear();
                tile_map.overlay.clear();

                let background_width = tile_map.background_width;
                let background_height = tile_map.background_height;
                let inverse_background_width = 1. / background_width as f32;
                let inverse_background_height = 1. / background_height as f32;

                let overlay_width = tile_map.overlay_width;
                let overlay_height = tile_map.overlay_height;
                let inverse_overlay_width = 1. / overlay_width as f32;
                let inverse_overlay_height = 1. / overlay_height as f32;

                for y in camera_r.top..camera_r.bottom {
                    for x in camera_r.left..camera_r.right {
                        if let Some(background_idx) = tile_map.tile_indices[max_x * y + x] {
                            tile_map.background.add(
                                ggez::graphics::DrawParam::default()
                                    .src(
                                        [
                                            (background_idx % background_width) as f32
                                                * inverse_background_width,
                                            (background_idx / background_width) as f32
                                                * inverse_background_height,
                                            inverse_background_width,
                                            inverse_background_height,
                                        ]
                                        .into(),
                                    )
                                    .dest([
                                        x as f32 * config::TILE_PIXELS_SIZE_F32,
                                        y as f32 * config::TILE_PIXELS_SIZE_F32,
                                    ]),
                            );
                        }

                        if let Some(overlay_idx) = tile_map.overlay_indices[max_x * y + x] {
                            tile_map.overlay.add(
                                ggez::graphics::DrawParam::default()
                                    .src(
                                        [
                                            (overlay_idx % overlay_width) as f32
                                                * inverse_overlay_width,
                                            (overlay_idx / overlay_width) as f32
                                                * inverse_overlay_height,
                                            inverse_overlay_width,
                                            inverse_overlay_height,
                                        ]
                                        .into(),
                                    )
                                    .dest([
                                        x as f32 * config::TILE_PIXELS_SIZE_F32,
                                        y as f32 * config::TILE_PIXELS_SIZE_F32,
                                    ]),
                            );
                        }
                    }
                }
            }
        }
    }
}

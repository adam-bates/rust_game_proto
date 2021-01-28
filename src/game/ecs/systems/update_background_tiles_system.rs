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

    #[tracing::instrument(skip(should_update_background_tiles_r, camera_r, tile_map_r), name = "UpdateBackgroundTilesSystem")]
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

                let sprite_sheet_width = tile_map.sprite_sheet_width;
                let inverse_sprite_sheet_width = 1. / tile_map.sprite_sheet_width as f32;
                let inverse_sprite_sheet_height = 1. / tile_map.sprite_sheet_height as f32;

                for y in camera_r.top..camera_r.bottom {
                    for x in camera_r.left..camera_r.right {
                        let tile_idx = tile_map.tile_indices[max_x * y + x];
                        tile_map.background.add(
                            ggez::graphics::DrawParam::default()
                                .src(
                                    [
                                        (tile_idx % sprite_sheet_width) as f32
                                            * inverse_sprite_sheet_width,
                                        (tile_idx / sprite_sheet_width) as f32
                                            * inverse_sprite_sheet_height,
                                        inverse_sprite_sheet_width,
                                        inverse_sprite_sheet_height,
                                    ]
                                    .into(),
                                )
                                .dest([
                                    x as f32 * config::TILE_PIXELS_SIZE_F32,
                                    y as f32 * config::TILE_PIXELS_SIZE_F32,
                                ]),
                        );

                        if let Some(overlay_idx) = tile_map.overlay_indices[max_x * y + x] {
                            tile_map.overlay.add(
                                ggez::graphics::DrawParam::default()
                                    .src(
                                        [
                                            (overlay_idx % sprite_sheet_width) as f32
                                                * inverse_sprite_sheet_width,
                                            (overlay_idx / sprite_sheet_width) as f32
                                                * inverse_sprite_sheet_height,
                                            inverse_sprite_sheet_width,
                                            inverse_sprite_sheet_height,
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

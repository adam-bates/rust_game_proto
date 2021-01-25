use super::{
    config,
    resources::{Camera, ShouldUpdateBackgroundTiles, TileMap},
};

pub struct UpdateBackgroundTilesSystem;

impl<'a> specs::System<'a> for UpdateBackgroundTilesSystem {
    type SystemData = (
        specs::Write<'a, ShouldUpdateBackgroundTiles>,
        specs::Read<'a, Camera>,
        Option<specs::Write<'a, TileMap>>,
    );

    fn run(
        &mut self,
        (mut should_update_background_tiles_r, camera_r, tile_map_r): Self::SystemData,
    ) {
        if should_update_background_tiles_r.0 {
            should_update_background_tiles_r.0 = false;

            if let Some(mut tile_map) = tile_map_r {
                // Update background tiles to draw
                let (max_x, max_y) = tile_map.dimensions();

                let left = (camera_r.x as isize - 1).max(0) as usize;
                let right =
                    (camera_r.x as usize + config::VIEWPORT_TILES_WIDTH_USIZE + 1).min(max_x);
                let top = (camera_r.y as isize - 1).max(0) as usize;
                let bottom =
                    (camera_r.y as usize + config::VIEWPORT_TILES_HEIGHT_USIZE + 1).min(max_y);

                tile_map.background.clear();

                let sprite_sheet_width = tile_map.sprite_sheet_width;
                let inverse_sprite_sheet_width = 1. / tile_map.sprite_sheet_width as f32;
                let inverse_sprite_sheet_height = 1. / tile_map.sprite_sheet_height as f32;
                for y in top..bottom {
                    for x in left..right {
                        let tile_idx = tile_map.tile_indices[max_x * y + x];
                        tile_map.background.add(
                            ggez::graphics::DrawParam::default()
                                .src(
                                    [
                                        ((tile_idx - 1) % sprite_sheet_width) as f32
                                            * inverse_sprite_sheet_width,
                                        ((tile_idx - 1) / sprite_sheet_width) as f32
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

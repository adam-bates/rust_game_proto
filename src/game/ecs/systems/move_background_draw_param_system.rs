use super::{
    config,
    resources::{Camera, TileMap},
};

pub struct MoveBackgroundDrawParamSystem;

impl<'a> specs::System<'a> for MoveBackgroundDrawParamSystem {
    type SystemData = (specs::Write<'a, Camera>, Option<specs::Write<'a, TileMap>>);

    fn run(&mut self, (mut camera_r, tile_map_r): Self::SystemData) {
        if let Some(mut tile_map_r) = tile_map_r {
            let (max_x, max_y) = tile_map_r.dimensions();

            camera_r.left = (camera_r.x as isize - 1).max(0) as usize;
            camera_r.right =
                (camera_r.x as usize + config::VIEWPORT_TILES_WIDTH_USIZE + 1).min(max_x);
            camera_r.top = (camera_r.y as isize - 1).max(0) as usize;
            camera_r.bottom =
                (camera_r.y as usize + config::VIEWPORT_TILES_HEIGHT_USIZE + 1).min(max_y);

            if let ggez::graphics::Transform::Values { ref mut dest, .. } =
                tile_map_r.background_param.trans
            {
                dest.x = -camera_r.x * config::TILE_PIXELS_SIZE_F32;
                dest.y = -camera_r.y * config::TILE_PIXELS_SIZE_F32;
            }
        }
    }
}

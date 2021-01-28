use super::{
    config,
    resources::{Camera, TileMap},
};

#[derive(Debug)]
pub struct MoveBackgroundDrawParamSystem;

impl<'a> specs::System<'a> for MoveBackgroundDrawParamSystem {
    type SystemData = (specs::Read<'a, Camera>, Option<specs::Write<'a, TileMap>>);

    #[tracing::instrument(skip(camera_r, tile_map_r), name = "MoveBackgroundDrawParamSystem")]
    fn run(&mut self, (camera_r, tile_map_r): Self::SystemData) {
        if let Some(mut tile_map_r) = tile_map_r {
            if let ggez::graphics::Transform::Values { ref mut dest, .. } =
                tile_map_r.background_param.trans
            {
                dest.x = -camera_r.x * config::TILE_PIXELS_SIZE_F32;
                dest.y = -camera_r.y * config::TILE_PIXELS_SIZE_F32;
            }
        }
    }
}

use super::{
    components::Drawable,
    resources::{Camera, TileMap},
};

#[derive(Debug)]
pub struct FillTileMapToDrawSystem;

impl<'a> specs::System<'a> for FillTileMapToDrawSystem {
    type SystemData = (
        specs::Read<'a, Camera>,
        Option<specs::Write<'a, TileMap>>,
        specs::ReadStorage<'a, Drawable>,
    );

    #[tracing::instrument(
        skip(camera_r, tile_map_r, drawable_c,),
        name = "FillTileMapToDrawSystem"
    )]
    fn run(&mut self, (camera_r, tile_map_r, drawable_c): Self::SystemData) {
        if let Some(mut tile_map_r) = tile_map_r {
            tile_map_r.to_draw = vec![];

            // Draw entities in order of y first to emulate z-axis
            for y in camera_r.top..camera_r.bottom {
                for x in camera_r.left..camera_r.right {
                    if let Some(entity) = tile_map_r.get_tile(x, y).entity {
                        if let Some(drawable) = drawable_c.get(entity) {
                            tile_map_r.to_draw.push(drawable.clone());
                        }
                    }
                }
            }
        }
    }
}

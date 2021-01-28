use super::{
    components::Timer,
    resources::{Camera, DeltaTime, ShouldUpdateBackgroundTiles, TileMap},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct AnimateBackgroundSystem {
    pub timer: Timer,
    pub frame_map: HashMap<usize, usize>,
}

impl<'a> specs::System<'a> for AnimateBackgroundSystem {
    type SystemData = (
        Option<specs::Write<'a, TileMap>>,
        specs::Write<'a, ShouldUpdateBackgroundTiles>,
        specs::Read<'a, Camera>,
        specs::Read<'a, DeltaTime>,
    );

    #[tracing::instrument(skip(
        tile_map_r,
        should_update_background_tiles_r,
        camera_r,
        delta_time_r
    ), name = "AnimateBackgroundSystem")]
    fn run(
        &mut self,
        (tile_map_r, mut should_update_background_tiles_r, camera_r, delta_time_r): Self::SystemData,
    ) {
        if let Some(mut tile_map_r) = tile_map_r {
            self.timer.tick(delta_time_r.secs);

            if self.timer.finished() {
                self.timer.reset();
                self.frame_map.clear();

                tile_map_r.animation.iter_mut().for_each(|frame| {
                    let key = frame.tile_ids[frame.idx];

                    frame.idx = (frame.idx + 1) % frame.tile_ids.len();
                    let value = frame.tile_ids[frame.idx];

                    self.frame_map.insert(key, value);
                });

                let (tile_map_width, _) = tile_map_r.dimensions();

                for y in camera_r.top..camera_r.bottom {
                    for x in camera_r.left..camera_r.right {
                        let tile_idx = &mut tile_map_r.tile_indices[tile_map_width * y + x];
                        if let Some(new_value) = self.frame_map.get(tile_idx) {
                            *tile_idx = *new_value;
                        }
                    }
                }

                should_update_background_tiles_r.0 = true;
            }
        }
    }
}

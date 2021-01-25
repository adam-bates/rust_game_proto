use super::{
    components::Timer,
    resources::{DeltaTime, ShouldUpdateBackgroundTiles, TileMap},
};
use std::collections::HashMap;

pub struct AnimateBackgroundSystem {
    pub timer: Timer,
}

impl<'a> specs::System<'a> for AnimateBackgroundSystem {
    type SystemData = (
        Option<specs::Write<'a, TileMap>>,
        specs::Write<'a, ShouldUpdateBackgroundTiles>,
        specs::Read<'a, DeltaTime>,
    );

    fn run(
        &mut self,
        (tile_map_r, mut should_update_background_tiles_r, delta_time_r): Self::SystemData,
    ) {
        if let Some(mut tile_map_r) = tile_map_r {
            self.timer.tick(delta_time_r.secs);

            if self.timer.finished() {
                self.timer.reset();

                let mut frame_map = HashMap::new();

                tile_map_r.animation.iter_mut().for_each(|frame| {
                    let key = frame.tile_ids[frame.idx] + 1;

                    frame.idx = (frame.idx + 1) % frame.tile_ids.len();
                    let value = frame.tile_ids[frame.idx] + 1;

                    frame_map.insert(key, value);
                });

                tile_map_r
                    .tile_indices
                    .iter_mut()
                    .filter(|tile_id| frame_map.contains_key(tile_id))
                    .for_each(|tile_id| {
                        *tile_id = *frame_map.get(tile_id).unwrap();
                    });

                should_update_background_tiles_r.0 = true;
            }
        }
    }
}

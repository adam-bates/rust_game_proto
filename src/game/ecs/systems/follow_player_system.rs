use super::{
    components::{CurrentPosition, Player},
    config,
    resources::{Camera, CameraBounds},
};
use config::VIEWPORT_TILES_WIDTH_F32;
use specs::Join;

pub struct FollowPlayerSystem;

impl<'a> specs::System<'a> for FollowPlayerSystem {
    type SystemData = (
        specs::WriteExpect<'a, Camera>,
        Option<specs::Read<'a, CameraBounds>>,
        specs::ReadStorage<'a, Player>,
        specs::ReadStorage<'a, CurrentPosition>,
    );

    fn run(
        &mut self,
        (mut camera_r, camera_bounds_r, player_c, current_position_c): Self::SystemData,
    ) {
        // Help linter
        #[cfg(debug_assertions)]
        let camera_r = &mut camera_r as &mut Camera;

        if let Some(camera_bounds_r) = camera_bounds_r {
            for (_, current_position) in (&player_c, &current_position_c).join() {
                // Help linter
                #[cfg(debug_assertions)]
                let current_position = current_position as &CurrentPosition;

                camera_r.x = nalgebra::clamp(
                    current_position.x - config::VIEWPORT_TILES_WIDTH_F32 / 2. + 0.5,
                    camera_bounds_r.min_x,
                    camera_bounds_r.max_x,
                );
                camera_r.y = nalgebra::clamp(
                    current_position.y - config::VIEWPORT_TILES_HEIGHT_F32 / 2. + 0.5,
                    camera_bounds_r.min_y,
                    camera_bounds_r.max_y,
                );
            }
        } else {
            for (_, current_position) in (&player_c, &current_position_c).join() {
                // Help linter
                #[cfg(debug_assertions)]
                let current_position = current_position as &CurrentPosition;

                camera_r.x = current_position.x;
                camera_r.y = current_position.y;
            }
        }
    }
}

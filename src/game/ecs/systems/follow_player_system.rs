use super::{
    components::{CurrentPosition, Player},
    resources::{Camera, CameraBounds},
};
use specs::Join;

pub struct FollowPlayerSystem;

impl<'a> specs::System<'a> for FollowPlayerSystem {
    type SystemData = (
        specs::WriteExpect<'a, Camera>,
        Option<specs::Read<'a, CameraBounds>>,
        specs::ReadStorage<'a, Player>,
        specs::ReadStorage<'a, CurrentPosition>,
    );

    fn run(&mut self, (mut camera, camera_bounds, player, current_position): Self::SystemData) {
        // Help linter
        #[cfg(debug_assertions)]
        let camera = &mut camera as &mut Camera;

        if let Some(camera_bounds) = camera_bounds {
            for (_, current_position) in (&player, &current_position).join() {
                // Help linter
                #[cfg(debug_assertions)]
                let current_position = current_position as &CurrentPosition;

                camera.x =
                    nalgebra::clamp(current_position.x, camera_bounds.min_x, camera_bounds.max_x);
                camera.y =
                    nalgebra::clamp(current_position.y, camera_bounds.min_y, camera_bounds.max_y);
            }
        } else {
            for (_, current_position) in (&player, &current_position).join() {
                // Help linter
                #[cfg(debug_assertions)]
                let current_position = current_position as &CurrentPosition;

                camera.x = current_position.x;
                camera.y = current_position.y;
            }
        }
    }
}

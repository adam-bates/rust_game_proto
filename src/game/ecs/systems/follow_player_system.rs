use super::{
    components::{Player, RealPosition},
    resources::{Camera, CameraBounds},
};
use specs::Join;

pub struct FollowPlayerSystem;

impl<'a> specs::System<'a> for FollowPlayerSystem {
    type SystemData = (
        specs::WriteExpect<'a, Camera>,
        Option<specs::Read<'a, CameraBounds>>,
        specs::ReadStorage<'a, Player>,
        specs::ReadStorage<'a, RealPosition>,
    );

    fn run(&mut self, (mut camera, camera_bounds, player, real_position): Self::SystemData) {
        if let Some(camera_bounds) = camera_bounds {
            for (_, real_position) in (&player, &real_position).join() {
                camera.x =
                    nalgebra::clamp(real_position.x, camera_bounds.min_x, camera_bounds.max_x);
                camera.y =
                    nalgebra::clamp(real_position.y, camera_bounds.min_y, camera_bounds.max_y);
            }
        } else {
            for (_, real_position) in (&player, &real_position).join() {
                camera.x = real_position.x;
                camera.y = real_position.y;
            }
        }
    }
}

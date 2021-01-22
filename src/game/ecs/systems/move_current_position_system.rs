use super::{
    components::{CurrentPosition, TargetPosition, Timer},
    resources::DeltaTime,
};
use specs::Join;

pub struct MoveCurrentPositionSystem;

impl<'a> specs::System<'a> for MoveCurrentPositionSystem {
    type SystemData = (
        specs::WriteStorage<'a, CurrentPosition>,
        specs::ReadStorage<'a, TargetPosition>,
        specs::WriteStorage<'a, Timer>,
        specs::Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut current_position, target_position, mut timer, delta): Self::SystemData) {
        for (current_position, target_position, timer) in
            (&mut current_position, &target_position, &mut timer).join()
        {
            if target_position.x as f32 != current_position.x
                || target_position.y as f32 != current_position.y
            {
                let delta_secs = delta.duration.as_secs_f32();
                let distance_percent =
                    nalgebra::clamp(delta_secs / (timer.duration() - timer.elapsed()), 0., 1.);

                timer.tick(delta_secs);

                current_position.lerp(target_position, distance_percent);
            }
        }
    }
}

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

    fn run(
        &mut self,
        (mut current_position_c, target_position_c, mut timer_c, delta_r): Self::SystemData,
    ) {
        for (current_position, target_position, timer) in
            (&mut current_position_c, &target_position_c, &mut timer_c).join()
        {
            // Help linter
            #[cfg(debug_assertions)]
            let current_position = current_position as &mut CurrentPosition;
            #[cfg(debug_assertions)]
            let target_position = target_position as &TargetPosition;
            #[cfg(debug_assertions)]
            let timer = timer as &mut Timer;

            if target_position.x as f32 != current_position.x
                || target_position.y as f32 != current_position.y
            {
                let delta_secs = delta_r.secs;
                let distance_percent =
                    nalgebra::clamp(delta_secs / (timer.duration() - timer.elapsed()), 0., 1.);

                timer.tick(delta_secs);

                current_position.lerp(target_position, distance_percent);
            }
        }
    }
}

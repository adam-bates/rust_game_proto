use super::{
    components::{CurrentPosition, TargetPosition, Timer},
    resources::DeltaTime,
};
use specs::Join;

pub struct MoveCurrentPositionSystem;

impl<'a> specs::System<'a> for MoveCurrentPositionSystem {
    type SystemData = (
        specs::WriteStorage<'a, CurrentPosition>,
        specs::WriteStorage<'a, TargetPosition>,
        specs::WriteStorage<'a, Timer>,
        specs::Read<'a, DeltaTime>,
    );

    fn run(
        &mut self,
        (mut current_position_c, mut target_position_c, mut timer_c, delta_r): Self::SystemData,
    ) {
        for (current_position, target_position, timer) in (
            &mut current_position_c,
            &mut target_position_c,
            &mut timer_c,
        )
            .join()
        {
            // Help linter
            #[cfg(debug_assertions)]
            let current_position = current_position as &mut CurrentPosition;
            #[cfg(debug_assertions)]
            let target_position = target_position as &mut TargetPosition;
            #[cfg(debug_assertions)]
            let timer = timer as &mut Timer;

            if timer.should_tick() {
                let delta_secs = delta_r.secs;

                timer.tick(delta_secs);

                let distance_percent =
                    nalgebra::clamp(delta_secs / (timer.duration() - timer.elapsed()), 0., 1.);

                current_position.lerp(target_position, distance_percent);

                if timer.finished() {
                    timer.set_should_tick(false);
                }
            } else if target_position.is_moving {
                target_position.is_moving = false;
            }
        }
    }
}

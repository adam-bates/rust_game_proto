use super::{
    components::{CurrentPosition, TargetPosition, Timer},
    resources::DeltaTime,
};
use specs::Join;

#[derive(Debug)]
pub struct MoveCurrentPositionSystem;

impl<'a> specs::System<'a> for MoveCurrentPositionSystem {
    type SystemData = (
        specs::WriteStorage<'a, CurrentPosition>,
        specs::WriteStorage<'a, TargetPosition>,
        specs::WriteStorage<'a, Timer>,
        specs::Read<'a, DeltaTime>,
    );

    #[tracing::instrument(
        skip(current_position_c, target_position_c, timer_c, delta_r),
        name = "MoveCurrentPositionSystem"
    )]
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
                timer.tick(delta_r.secs);

                if timer.finished() {
                    current_position.x = target_position.x as f32;
                    current_position.y = target_position.y as f32;

                    target_position.from_x = target_position.x;
                    target_position.from_y = target_position.y;

                    timer.set_should_tick(false);
                } else {
                    *current_position = target_position.get_current_position(timer);
                }
            } else if target_position.is_moving {
                target_position.is_moving = false;
            }
        }
    }
}

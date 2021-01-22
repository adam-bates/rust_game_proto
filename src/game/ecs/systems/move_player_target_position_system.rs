use super::{
    components::{CurrentPosition, Player, TargetPosition, Timer},
    input::types::GameDirection,
    resources::PlayerMovementRequest,
};
use specs::Join;

const MAX_X: i32 = 24; // TODO
const MAX_Y: i32 = 20; // TODO

pub struct MovePlayerTargetPositionSystem;

type SystemData<'a> = (
    specs::Read<'a, PlayerMovementRequest>,
    specs::ReadStorage<'a, Player>,
    specs::ReadStorage<'a, CurrentPosition>,
    specs::WriteStorage<'a, TargetPosition>,
    specs::WriteStorage<'a, Timer>,
);

fn handle_input<'a>(
    (
        _player_movement_request,
        player_component,
        current_position_component,
        mut target_position_component,
        mut timer_component,
    ): SystemData<'a>,
    direction: &GameDirection,
) {
    let (direction_x, direction_y) = direction.to_xy();

    for (_, current_position, mut target_position, timer) in (
        &player_component,
        &current_position_component,
        &mut target_position_component,
        &mut timer_component,
    )
        .join()
    {
        if timer.finished() {
            target_position.x =
                nalgebra::clamp(current_position.x as i32 + direction_x, 0, MAX_X) as u32;

            target_position.y =
                nalgebra::clamp(current_position.y as i32 + direction_y, 0, MAX_Y) as u32;

            let is_collision = false; // TODO

            if is_collision {
                target_position.x = current_position.x as u32;
                target_position.y = current_position.y as u32;
            }

            // for (_, other_current_position, other_target_position) in (
            //     !&player_component,
            //     &current_position_component,
            //     &target_position_component,
            // )
            //     .join()
            // {
            //     //
            // }

            if target_position.x != current_position.x as u32
                || target_position.y != current_position.y as u32
            {
                println!(
                    "Moving from [{}, {}] to [{}, {}]",
                    current_position.x, current_position.y, target_position.x, target_position.y
                );
                timer.reset();
            }
        }
    }
}

// TODO: Handle collision
// TODO: Only move when direction was requested
impl<'a> specs::System<'a> for MovePlayerTargetPositionSystem {
    type SystemData = SystemData<'a>;

    fn run(
        &mut self,
        (
            player_movement_request,
            player_component,
            current_position_component,
            target_position_component,
            timer_component,
        ): Self::SystemData,
    ) {
        if let Some(direction) = player_movement_request.last_requested_direction {
            handle_input(
                (
                    player_movement_request,
                    player_component,
                    current_position_component,
                    target_position_component,
                    timer_component,
                ),
                &direction,
            );
        } else if let Some(direction) = player_movement_request.last_requested_x_direction {
            handle_input(
                (
                    player_movement_request,
                    player_component,
                    current_position_component,
                    target_position_component,
                    timer_component,
                ),
                &direction,
            );
        } else if let Some(direction) = player_movement_request.last_requested_y_direction {
            handle_input(
                (
                    player_movement_request,
                    player_component,
                    current_position_component,
                    target_position_component,
                    timer_component,
                ),
                &direction,
            );
        }
    }
}

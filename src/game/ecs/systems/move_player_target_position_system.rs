use super::{
    components::{CurrentPosition, Player, TargetPosition, Timer},
    config,
    input::types::GameDirection,
    resources::{Camera, PlayerMovementRequest, ShouldUpdateBackgroundTiles, TileMap},
};
use specs::Join;

pub struct MovePlayerTargetPositionSystem;

type SystemData<'a> = (
    Option<specs::Write<'a, TileMap>>,
    specs::Read<'a, PlayerMovementRequest>,
    specs::Read<'a, Camera>,
    specs::Write<'a, ShouldUpdateBackgroundTiles>,
    specs::ReadStorage<'a, Player>,
    specs::ReadStorage<'a, CurrentPosition>,
    specs::WriteStorage<'a, TargetPosition>,
    specs::WriteStorage<'a, Timer>,
);

fn handle_input<'a>(
    (
        mut tile_map_r,
        _player_movement_request_r,
        camera_r,
        mut should_update_background_tiles_r,
        player_c,
        current_position_c,
        mut target_position_c,
        mut timer_c,
    ): SystemData<'a>,
    direction: &GameDirection,
) {
    if let Some(tile_map) = &mut tile_map_r {
        let (direction_x, direction_y) = direction.to_xy();

        for (_, current_position, timer) in (&player_c, &current_position_c, &mut timer_c).join() {
            // Help linter
            #[cfg(debug_assertions)]
            let current_position = current_position as &CurrentPosition;
            #[cfg(debug_assertions)]
            let timer = timer as &mut Timer;

            let mut set_target_position = None;
            if timer.finished() {
                let tile_map_dimensions = tile_map.dimensions();

                let target_position_x = nalgebra::clamp(
                    current_position.x as isize + direction_x,
                    0,
                    tile_map_dimensions.0 as isize - 1,
                ) as usize;

                let target_position_y = nalgebra::clamp(
                    current_position.y as isize + direction_y,
                    0,
                    tile_map_dimensions.1 as isize - 1,
                ) as usize;

                let target_tile = tile_map.get_tile(target_position_x, target_position_y);

                // Can't walk on tile
                if target_tile.tile_type.is_some() {
                    return;
                }

                // Another entity is already in the target location
                if target_tile.entity.is_some() {
                    return;
                }

                if target_position_x != current_position.x as usize
                    || target_position_y != current_position.y as usize
                {
                    let player_entity = tile_map
                        .get_tile_mut(current_position.x as usize, current_position.y as usize)
                        .entity
                        .take()
                        .expect("Player entity isn't in tile_map @ current_position");

                    tile_map
                        .get_tile_mut(target_position_x, target_position_y)
                        .entity
                        .replace(player_entity);

                    set_target_position = Some((target_position_x, target_position_y));
                    timer.reset();

                    should_update_background_tiles_r.0 = true;
                }
            }

            // We need to do this mutation separately in order to run the query on non-player's target_positions above
            if let Some(set_target_position) = set_target_position {
                for (_, target_position) in (&player_c, &mut target_position_c).join() {
                    // Help linter
                    #[cfg(debug_assertions)]
                    let target_position = target_position as &mut TargetPosition;

                    target_position.x = set_target_position.0;
                    target_position.y = set_target_position.1;
                }
            }
        }
    }
}

impl<'a> specs::System<'a> for MovePlayerTargetPositionSystem {
    type SystemData = SystemData<'a>;

    fn run(
        &mut self,
        (
            tile_map_r,
            player_movement_request_r,
            camera_r,
            mut should_update_background_tiles_r,
            player_c,
            current_position_c,
            target_position_c,
            timer_c,
        ): Self::SystemData,
    ) {
        // Last requested direction
        if let Some(direction) = player_movement_request_r.last_requested_direction {
            handle_input(
                (
                    tile_map_r,
                    player_movement_request_r,
                    camera_r,
                    should_update_background_tiles_r,
                    player_c,
                    current_position_c,
                    target_position_c,
                    timer_c,
                ),
                &direction,
            );

        // Last requested x direction
        } else if let Some(direction) = player_movement_request_r.last_requested_x_direction {
            handle_input(
                (
                    tile_map_r,
                    player_movement_request_r,
                    camera_r,
                    should_update_background_tiles_r,
                    player_c,
                    current_position_c,
                    target_position_c,
                    timer_c,
                ),
                &direction,
            );

        // Last requested y direction
        } else if let Some(direction) = player_movement_request_r.last_requested_y_direction {
            handle_input(
                (
                    tile_map_r,
                    player_movement_request_r,
                    camera_r,
                    should_update_background_tiles_r,
                    player_c,
                    current_position_c,
                    target_position_c,
                    timer_c,
                ),
                &direction,
            );
        }
    }
}

use super::{
    components::{CurrentPosition, FacingDirection, Player, SpriteSheet, TargetPosition, Timer},
    config,
    input::types::GameDirection,
    maps::TileType,
    resources::{PlayerMovementRequest, ShouldUpdateBackgroundTiles, TileMap},
};
use specs::Join;

#[derive(Debug)]
pub struct MovePlayerTargetPositionSystem;

type SystemData<'a> = (
    Option<specs::Write<'a, TileMap>>,
    specs::Read<'a, PlayerMovementRequest>,
    specs::Write<'a, ShouldUpdateBackgroundTiles>,
    specs::ReadStorage<'a, Player>,
    specs::ReadStorage<'a, CurrentPosition>,
    specs::WriteStorage<'a, TargetPosition>,
    specs::WriteStorage<'a, Timer>,
    specs::WriteStorage<'a, SpriteSheet>,
    specs::WriteStorage<'a, FacingDirection>,
);

fn move_target_position<'a>(
    should_update_background_tiles_r: &mut ShouldUpdateBackgroundTiles,
    tile_map: &mut TileMap,
    current_position: &CurrentPosition,
    target_position: &mut TargetPosition,
    timer: &mut Timer,
    sprite_sheet: &mut SpriteSheet,
    direction: &GameDirection,
) {
    let (direction_x, direction_y) = direction.to_xy();

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

    // Another entity is already in the target location
    if target_tile.entity.is_some() {
        return;
    }

    // Can't walk on tile
    if let Some(tile_type) = &target_tile.tile_type {
        match tile_type {
            TileType::Wall | TileType::Water => return,
        }
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

        timer.reset();
        timer.set_should_tick(true);

        should_update_background_tiles_r.0 = true;

        target_position.x = target_position_x;
        target_position.y = target_position_y;
        target_position.is_moving = true;

        sprite_sheet.idx = match direction {
            GameDirection::Down => config::ENTITY_SPRITE_SHEET_IDX_WALK_DOWN,
            GameDirection::Right => config::ENTITY_SPRITE_SHEET_IDX_WALK_RIGHT,
            GameDirection::Up => config::ENTITY_SPRITE_SHEET_IDX_WALK_UP,
            GameDirection::Left => config::ENTITY_SPRITE_SHEET_IDX_WALK_LEFT,
        };
        sprite_sheet.refresh();
    }
}

fn handle_input<'a>(
    (
        mut tile_map_r,
        _player_movement_request_r,
        mut should_update_background_tiles_r,
        player_c,
        current_position_c,
        mut target_position_c,
        mut timer_c,
        mut sprite_sheet_c,
        mut facing_direction_c,
    ): SystemData<'a>,
    direction: &GameDirection,
) {
    if let Some(tile_map) = &mut tile_map_r {
        for (_, current_position, target_position, timer, sprite_sheet, facing_direction) in (
            &player_c,
            &current_position_c,
            &mut target_position_c,
            &mut timer_c,
            &mut sprite_sheet_c,
            &mut facing_direction_c,
        )
            .join()
        {
            // Help linter
            #[cfg(debug_assertions)]
            let current_position = current_position as &CurrentPosition;
            #[cfg(debug_assertions)]
            let target_position = target_position as &mut TargetPosition;
            #[cfg(debug_assertions)]
            let timer = timer as &mut Timer;
            #[cfg(debug_assertions)]
            let facing_direction = facing_direction as &mut FacingDirection;

            if timer.finished() {
                if target_position.is_moving || facing_direction.direction == *direction {
                    facing_direction.direction = *direction;

                    move_target_position(
                        &mut should_update_background_tiles_r,
                        tile_map,
                        current_position,
                        target_position,
                        timer,
                        sprite_sheet,
                        direction,
                    );
                }
            }
        }
    }
}

impl<'a> specs::System<'a> for MovePlayerTargetPositionSystem {
    type SystemData = SystemData<'a>;

    #[tracing::instrument(skip(
        tile_map_r,
        player_movement_request_r,
        should_update_background_tiles_r,
        player_c,
        current_position_c,
        target_position_c,
        timer_c,
        sprite_sheet_c,
        facing_direction_c,
    ), name = "MovePlayerTargetPositionSystem")]
    fn run(
        &mut self,
        (
            tile_map_r,
            player_movement_request_r,
            should_update_background_tiles_r,
            player_c,
            current_position_c,
            target_position_c,
            timer_c,
            mut sprite_sheet_c,
            mut facing_direction_c,
        ): Self::SystemData,
    ) {
        // Last requested direction
        if let Some(direction) = player_movement_request_r.last_requested_direction {
            handle_input(
                (
                    tile_map_r,
                    player_movement_request_r,
                    should_update_background_tiles_r,
                    player_c,
                    current_position_c,
                    target_position_c,
                    timer_c,
                    sprite_sheet_c,
                    facing_direction_c,
                ),
                &direction,
            );

        // Last requested x direction
        } else if let Some(direction) = player_movement_request_r.last_requested_x_direction {
            handle_input(
                (
                    tile_map_r,
                    player_movement_request_r,
                    should_update_background_tiles_r,
                    player_c,
                    current_position_c,
                    target_position_c,
                    timer_c,
                    sprite_sheet_c,
                    facing_direction_c,
                ),
                &direction,
            );

        // Last requested y direction
        } else if let Some(direction) = player_movement_request_r.last_requested_y_direction {
            handle_input(
                (
                    tile_map_r,
                    player_movement_request_r,
                    should_update_background_tiles_r,
                    player_c,
                    current_position_c,
                    target_position_c,
                    timer_c,
                    sprite_sheet_c,
                    facing_direction_c,
                ),
                &direction,
            );
        } else {
            for (_, sprite_sheet, facing_direction) in
                (&player_c, &mut sprite_sheet_c, &mut facing_direction_c).join()
            {
                // Help linter
                #[cfg(debug_assertions)]
                let sprite_sheet = sprite_sheet as &mut SpriteSheet;
                #[cfg(debug_assertions)]
                let facing_direction = facing_direction as &mut FacingDirection;

                sprite_sheet.idx = match facing_direction.direction {
                    GameDirection::Down => config::ENTITY_SPRITE_SHEET_IDX_IDLE_DOWN,
                    GameDirection::Left => config::ENTITY_SPRITE_SHEET_IDX_IDLE_LEFT,
                    GameDirection::Up => config::ENTITY_SPRITE_SHEET_IDX_IDLE_UP,
                    GameDirection::Right => config::ENTITY_SPRITE_SHEET_IDX_IDLE_RIGHT,
                };
                sprite_sheet.refresh();
            }
        }
    }
}

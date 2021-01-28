use super::{
    config,
    ecs::{
        components::{
            CurrentPosition, Drawable, FacingDirection, Player, SpriteRow, SpriteSheet,
            TargetPosition, Timer,
        },
        resources::{Camera, PlayerMovementRequest, ShouldUpdateBackgroundTiles},
        systems::{
            AnimateBackgroundSystem, FillTileMapToDrawSystem, FollowPlayerSystem,
            MoveBackgroundDrawParamSystem, MoveCurrentPositionSystem,
            MovePlayerTargetPositionSystem, UpdateBackgroundTilesSystem, UpdateDrawParamSystem,
            UpdateSpriteSheetDrawParamSystem,
        },
    },
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameButton, GameDirection, GameInput},
    types::{Scene, SceneSwitch},
};
use specs::{Builder, Join, WorldExt};
use std::sync::Arc;

pub struct OverworldScene {
    dispatcher: specs::Dispatcher<'static, 'static>,
    entities: Vec<specs::Entity>,
}

impl OverworldScene {
    pub fn new(game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        // TODO: Build from loaded save file

        let player_target_position = TargetPosition::default();
        let player_current_position = CurrentPosition {
            x: player_target_position.x as f32,
            y: player_target_position.y as f32,
        };

        game_state.world.register::<Player>();
        game_state.world.register::<CurrentPosition>();
        game_state.world.register::<TargetPosition>();
        game_state.world.register::<Timer>();
        game_state.world.register::<Drawable>();
        game_state.world.register::<FacingDirection>();
        game_state.world.register::<SpriteSheet>();
        game_state.world.insert(PlayerMovementRequest::default());
        game_state.world.insert(Camera {
            x: player_target_position.x as f32,
            y: player_target_position.y as f32,
            ..Default::default()
        });
        game_state.world.insert(ShouldUpdateBackgroundTiles(true));

        let dispatcher = specs::DispatcherBuilder::new()
            .with(
                MovePlayerTargetPositionSystem,
                "move_player_target_position_system",
                &[],
            )
            .with(
                MoveCurrentPositionSystem,
                "move_current_position_system",
                &["move_player_target_position_system"],
            )
            .with(
                FollowPlayerSystem,
                "follow_player_system",
                &["move_current_position_system"],
            )
            .with(
                UpdateDrawParamSystem,
                "update_draw_param_system",
                &["follow_player_system"],
            )
            .with(
                MoveBackgroundDrawParamSystem,
                "move_background_draw_param_system",
                &["follow_player_system"],
            )
            .with(
                UpdateSpriteSheetDrawParamSystem,
                "update_sprite_sheet_draw_param_system",
                &["move_player_target_position_system"],
            )
            .with(
                FillTileMapToDrawSystem,
                "fill_tile_map_to_draw_system",
                &[
                    "follow_player_system",
                    "update_sprite_sheet_draw_param_system",
                ],
            )
            .with(
                AnimateBackgroundSystem {
                    timer: Timer::new(std::time::Duration::from_secs_f32(0.5), true),
                    frame_map: std::collections::HashMap::default(),
                },
                "animate_background_system",
                &[],
            )
            .with(
                UpdateBackgroundTilesSystem,
                "update_background_tiles_system",
                &[
                    "animate_background_system",
                    "move_player_target_position_system",
                    "follow_player_system",
                ],
            )
            .build();

        let player_entity = game_state
            .world
            .create_entity()
            .with(Player)
            .with(player_current_position)
            .with(player_target_position)
            .with(Timer {
                duration: config::WALK_SECONDS_PER_TILE,
                repeating: true,
                elapsed: 0.0,
                finished: true, // Start finished to allow movement
                should_tick: false,
            })
            .with(Drawable {
                drawable: Arc::new(ggez::graphics::Mesh::new_rectangle(
                    ctx,
                    ggez::graphics::DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        0.,
                        config::TILE_PIXELS_SIZE_F32 - 24.,
                        config::TILE_PIXELS_SIZE_F32,
                        24.,
                    ),
                    ggez::graphics::Color::from_rgb(200, 50, 50),
                )?),
                draw_params: ggez::graphics::DrawParam::default(),
            })
            .with(SpriteSheet::new(vec![
                SpriteRow::new(1), // IDLE DOWN
                SpriteRow::new(1), // IDLE RIGHT
                SpriteRow::new(1), // IDLE UP
                SpriteRow::new(1), // IDLE LEFT
                SpriteRow::new(1), // WALK DOWN
                SpriteRow::new(1), // WALK RIGHT
                SpriteRow::new(1), // WALK UP
                SpriteRow::new(1), // WALK LEFT
            ]))
            .with(FacingDirection {
                direction: GameDirection::Down,
            })
            .build();

        Ok(Self {
            dispatcher,
            entities: vec![player_entity],
        })
    }

    // TODO: Function to build from save file given a filesystem
}

impl std::fmt::Debug for OverworldScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {{ ... }}", self.name()))
    }
}

impl Scene for OverworldScene {
    fn dispose(&mut self, game_state: &mut GameState, _ctx: &mut ggez::Context) -> GameResult {
        game_state.world.remove::<Camera>();
        game_state.world.remove::<PlayerMovementRequest>();

        if let Err(e) = game_state.world.delete_entities(self.entities.as_slice()) {
            return Err(ggez::GameError::CustomError(format!(
                "Wrong generation error when deleting entities in OverworldScene::dispose: {}",
                e
            )));
        }

        Ok(())
    }

    #[tracing::instrument]
    fn update(
        &mut self,
        game_state: &mut GameState,
        _ctx: &mut ggez::Context,
        _delta_secs: f32,
    ) -> GameResult<Option<SceneSwitch>> {
        self.dispatcher.dispatch(&game_state.world);
        Ok(None)
    }

    #[tracing::instrument]
    fn draw(&self, _game_state: &GameState, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn input(
        &mut self,
        game_state: &mut GameState,
        _ctx: &mut ggez::Context,
        input: GameInput,
    ) -> GameResult<Option<SceneSwitch>> {
        let mut direction_to_turn = None;

        if let Some(player_movement_request) = game_state.world.get_mut::<PlayerMovementRequest>() {
            match input {
                GameInput::Button { button, pressed } => match button {
                    GameButton::Up => {
                        player_movement_request.last_requested_direction = if pressed {
                            Some(GameDirection::Up)
                        } else {
                            None
                        };
                        player_movement_request.last_requested_y_direction =
                            player_movement_request.last_requested_direction.clone();

                        direction_to_turn = player_movement_request.last_requested_direction;
                    }
                    GameButton::Down => {
                        player_movement_request.last_requested_direction = if pressed {
                            Some(GameDirection::Down)
                        } else {
                            None
                        };
                        player_movement_request.last_requested_y_direction =
                            player_movement_request.last_requested_direction.clone();

                        direction_to_turn = player_movement_request.last_requested_direction;
                    }
                    GameButton::Left => {
                        player_movement_request.last_requested_direction = if pressed {
                            Some(GameDirection::Left)
                        } else {
                            None
                        };
                        player_movement_request.last_requested_x_direction =
                            player_movement_request.last_requested_direction.clone();

                        direction_to_turn = player_movement_request.last_requested_direction;
                    }
                    GameButton::Right => {
                        player_movement_request.last_requested_direction = if pressed {
                            Some(GameDirection::Right)
                        } else {
                            None
                        };
                        player_movement_request.last_requested_x_direction =
                            player_movement_request.last_requested_direction.clone();

                        direction_to_turn = player_movement_request.last_requested_direction;
                    }
                    _ => {}
                },
                GameInput::Direction { direction } => {
                    player_movement_request.last_requested_direction = direction;

                    direction_to_turn = player_movement_request.last_requested_direction;
                }
            }
        }

        if let Some(direction) = direction_to_turn {
            let (player_c, target_position_c, mut timer_c, mut facing_direction_c): (
                specs::ReadStorage<Player>,
                specs::ReadStorage<TargetPosition>,
                specs::WriteStorage<Timer>,
                specs::WriteStorage<FacingDirection>,
            ) = game_state.world.system_data();

            for (_, target_position, timer, facing_direction) in (
                &player_c,
                &target_position_c,
                &mut timer_c,
                &mut facing_direction_c,
            )
                .join()
            {
                // Help linter
                #[cfg(debug_assertions)]
                let target_position = target_position as &TargetPosition;
                #[cfg(debug_assertions)]
                let timer = timer as &mut Timer;
                #[cfg(debug_assertions)]
                let facing_direction = facing_direction as &mut FacingDirection;

                if !target_position.is_moving && facing_direction.direction != direction {
                    facing_direction.direction = direction;

                    timer.reset();
                    timer.elapsed = timer.duration - config::WAIT_AFTER_TURN_BEFORE_MOVE;
                    timer.set_should_tick(true);
                }
            }
        }

        Ok(None)
    }

    fn should_update_previous(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        "OverworldScene"
    }
}

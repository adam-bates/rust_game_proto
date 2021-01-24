use super::{
    config,
    ecs::{
        components::{CurrentPosition, Drawable, Player, TargetPosition, Timer},
        resources::{Camera, PlayerMovementRequest},
        systems::{
            FollowPlayerSystem, MoveBackgroundDrawParamSystem, MoveCurrentPositionSystem,
            MovePlayerTargetPositionSystem, UpdateDrawParamSystem,
        },
    },
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameButton, GameDirection, GameInput},
    types::{Scene, SceneSwitch},
};
use specs::{Builder, WorldExt};

pub struct OverworldScene {
    dispatcher: specs::Dispatcher<'static, 'static>,
    entities: Vec<specs::Entity>,
}

impl OverworldScene {
    pub fn new(game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        // TODO: Build from loaded save file

        let player_target_position = TargetPosition { x: 0, y: 0 };
        let player_current_position = CurrentPosition {
            x: player_target_position.x as f32,
            y: player_target_position.y as f32,
        };

        game_state.world.register::<Player>();
        game_state.world.register::<CurrentPosition>();
        game_state.world.register::<TargetPosition>();
        game_state.world.register::<Timer>();
        game_state.world.register::<Drawable>();
        game_state.world.insert(PlayerMovementRequest::default());
        game_state.world.insert(Camera {
            x: player_target_position.x as f32,
            y: player_target_position.y as f32,
            ..Default::default()
        });

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
            .build();

        let player_entity = game_state
            .world
            .create_entity()
            .with(Player)
            .with(player_current_position)
            .with(player_target_position)
            .with(Timer {
                duration: 0.2,
                repeating: true,
                elapsed: 0.0,
                finished: true, // Start finished to allow movement
            })
            .with(Drawable {
                drawable: Box::new(ggez::graphics::Mesh::new_rectangle(
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
            .build();

        Ok(Self {
            dispatcher,
            entities: vec![player_entity],
        })
    }

    // TODO: Function to build from save file given a filesystem
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

    fn update(
        &mut self,
        game_state: &mut GameState,
        _ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>> {
        self.dispatcher.dispatch(&game_state.world);
        Ok(None)
    }

    fn draw(&self, _game_state: &GameState, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn input(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        input: GameInput,
    ) -> GameResult<Option<SceneSwitch>> {
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
                    }
                    GameButton::Down => {
                        player_movement_request.last_requested_direction = if pressed {
                            Some(GameDirection::Down)
                        } else {
                            None
                        };
                        player_movement_request.last_requested_y_direction =
                            player_movement_request.last_requested_direction.clone();
                    }
                    GameButton::Left => {
                        player_movement_request.last_requested_direction = if pressed {
                            Some(GameDirection::Left)
                        } else {
                            None
                        };
                        player_movement_request.last_requested_x_direction =
                            player_movement_request.last_requested_direction.clone();
                    }
                    GameButton::Right => {
                        player_movement_request.last_requested_direction = if pressed {
                            Some(GameDirection::Right)
                        } else {
                            None
                        };
                        player_movement_request.last_requested_x_direction =
                            player_movement_request.last_requested_direction.clone();
                    }
                    _ => {}
                },
                GameInput::Direction { direction } => {
                    player_movement_request.last_requested_direction = direction;
                }
            }
        }

        Ok(None)
    }

    fn should_update_previous(&self) -> bool {
        true
    }
}

use super::{
    ecs::{
        components::{CurrentPosition, Player, TargetPosition, Timer},
        resources::{Camera, CameraBounds, DeltaTime, PlayerMovementRequest},
        systems::{FollowPlayerSystem, MoveCurrentPositionSystem, MovePlayerTargetPositionSystem},
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

        let player_target_position = TargetPosition { x: 5, y: 7 };
        let player_current_position = CurrentPosition {
            x: player_target_position.x as f32,
            y: player_target_position.y as f32,
        };

        game_state.world.register::<Player>();
        game_state.world.register::<CurrentPosition>();
        game_state.world.register::<TargetPosition>();
        game_state.world.register::<Timer>();
        game_state.world.insert(PlayerMovementRequest::default());
        game_state.world.insert(DeltaTime::default());
        game_state.world.insert(Camera {
            x: player_target_position.x as f32,
            y: player_target_position.y as f32,
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
                &[],
            )
            .with(FollowPlayerSystem, "follow_player_system", &[])
            // .with(PrintSystem, "print_system", &["follow_player_system"])
            .build();
        // dispatcher.setup(&mut game_state.world);

        let player_entity = game_state
            .world
            .create_entity()
            .with(Player)
            .with(player_current_position)
            .with(player_target_position)
            .with(Timer {
                duration: 0.5,
                repeating: true,
                elapsed: 0.0,
                finished: true, // Start finished to allow movement
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
    fn dispose(&mut self, game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult {
        game_state.world.remove::<Camera>();
        game_state.world.remove::<DeltaTime>();
        game_state.world.remove::<PlayerMovementRequest>();
        game_state.world.delete_entities(self.entities.as_slice());
        Ok(())
    }

    fn update(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>> {
        // println!("OverworldScene::update");
        self.dispatcher.dispatch(&game_state.world);
        Ok(None)
    }

    fn draw(&self, game_state: &GameState, ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn input(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        input: GameInput,
    ) -> GameResult<Option<SceneSwitch>> {
        match input {
            GameInput::Button { button, pressed } => {
                if pressed {
                    match button {
                        GameButton::Start => game_state.world.insert(CameraBounds {
                            min_x: 0.,
                            min_y: 0.,
                            max_x: 2.,
                            max_y: 3.,
                        }),
                        GameButton::Select => {
                            game_state.world.remove::<CameraBounds>();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

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

use super::{
    config,
    ecs::{
        components::{
            CurrentPosition, Drawable, FacingDirection, Id, Interactable, MapName, Player,
            SpriteRow, SpriteSheet, TargetPosition, Timer,
        },
        resources::{Camera, PlayerMovementRequest, ShouldUpdateBackgroundTiles, TileMap},
        systems::{
            AnimateSystem, FillTileMapToDrawSystem, FollowPlayerSystem,
            MoveBackgroundDrawParamSystem, MoveCurrentPositionSystem,
            MovePlayerTargetPositionSystem, UpdateBackgroundTilesSystem, UpdateDrawParamSystem,
            UpdateSpriteSheetDrawParamSystem,
        },
    },
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameButton, GameDirection, GameInput},
    save::MetaSaveData,
    types::{Scene, SceneBuilder, SceneSwitch},
    utils, PalletTownOverworldScene, PauseMenuScene,
};
use ggez::graphics::Drawable as GgezDrawable;
use specs::{Builder, Join, WorldExt};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc, sync::Arc};

const PLAYER_FILE: &str = "/spritesheets/entities/player.png";

pub struct OverworldScene {
    dispatcher: specs::Dispatcher<'static, 'static>,
    entities: Vec<specs::Entity>,
    map_builders: HashMap<MapName, SceneBuilder>,
}

impl OverworldScene {
    pub fn new(game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        // TODO: Build from loaded save file

        let player_target_position = TargetPosition::default();
        let player_current_position = CurrentPosition {
            x: player_target_position.x as f32,
            y: player_target_position.y as f32,
        };

        game_state.world.register::<Id>();
        game_state.world.register::<Player>();
        game_state.world.register::<CurrentPosition>();
        game_state.world.register::<TargetPosition>();
        game_state.world.register::<Timer>();
        game_state.world.register::<Drawable>();
        game_state.world.register::<FacingDirection>();
        game_state.world.register::<SpriteSheet>();
        game_state.world.register::<Interactable>();
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
                AnimateSystem {
                    timer: Timer::new(std::time::Duration::from_secs_f32(0.5), true),
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

        let player_image = ggez::graphics::Image::new(ctx, PathBuf::from(PLAYER_FILE))?;
        let player_spritesheet = SpriteSheet::new(vec![
            SpriteRow::new(2), // IDLE DOWN
            SpriteRow::new(2), // IDLE RIGHT
            SpriteRow::new(2), // IDLE UP
            SpriteRow::new(2), // IDLE LEFT
            SpriteRow::new(2), // WALK DOWN
            SpriteRow::new(2), // WALK RIGHT
            SpriteRow::new(2), // WALK UP
            SpriteRow::new(2), // WALK LEFT
        ]);

        let player_width = player_image.width() as f32 / player_spritesheet.row().frames as f32;
        let player_height =
            player_image.height() as f32 / player_spritesheet.sprite_rows.len() as f32;

        let player_offset_x = (player_width - config::TILE_PIXELS_SIZE_F32) / (player_width * 2.);
        let player_offset_y = (player_height - config::TILE_PIXELS_SIZE_F32) / player_height;

        // IDK why these numbers work but they make the sprites pixel-precise when offset
        // Otherwise the offset isn't correct and pixels bleed past where they should
        // (noticeable when the bottom pixels bleed past an overlay sprite)
        const OFFSET_FIX_X: f32 = -0.001;
        const OFFSET_FIX_Y: f32 = 0.02;

        // Bottom of image should be level with floor
        // And sides of image should be centered
        let player_draw_param = ggez::graphics::DrawParam::default().offset([
            player_offset_x + OFFSET_FIX_X,
            player_offset_y + OFFSET_FIX_Y,
        ]);

        let player_entity = game_state
            .world
            .create_entity()
            .with(Id::new("Player"))
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
                drawable: Arc::new(player_image),
                draw_params: player_draw_param,
            })
            .with(player_spritesheet)
            .with(FacingDirection {
                direction: GameDirection::Down,
            })
            .build();

        let pallet_town_builder: SceneBuilder = Box::new(|game_state, ctx| {
            let scene = PalletTownOverworldScene::new(game_state, ctx)?;
            Ok(Rc::new(RefCell::new(scene)))
        });

        let map_builders = utils::map!(
            MapName::PalletTown => pallet_town_builder,
        );

        Ok(Self {
            dispatcher,
            entities: vec![player_entity],
            map_builders,
        })
    }
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

    fn on_create(
        &mut self,
        game_state: &mut GameState,
        _ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>> {
        let meta_data = game_state.world.fetch::<MetaSaveData>();
        let scene_builder: SceneBuilder = meta_data.current_map.scene_builder();

        Ok(Some(SceneSwitch::Push(scene_builder)))
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
    fn draw(&self, game_state: &GameState, ctx: &mut ggez::Context) -> GameResult {
        if let Some(tile_map) = game_state.world.try_fetch::<TileMap>() {
            tile_map.background.draw(ctx, tile_map.spritesheet_param)?;

            for drawable in &tile_map.to_draw {
                drawable.drawable.draw(ctx, drawable.draw_params)?;
            }

            tile_map.overlay.draw(ctx, tile_map.spritesheet_param)?;
        }

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
                    GameButton::Start => {
                        if pressed {
                            let scene_builder: SceneBuilder = Box::new(|_, _| {
                                let scene = PauseMenuScene::new();
                                Ok(Rc::new(RefCell::new(scene)))
                            });

                            player_movement_request.last_requested_direction = None;
                            player_movement_request.last_requested_x_direction = None;
                            player_movement_request.last_requested_y_direction = None;

                            return Ok(Some(SceneSwitch::Push(scene_builder)));
                        }
                    }
                    GameButton::Primary => {
                        if pressed {
                            if let Some(tile_map) = game_state.world.try_fetch::<TileMap>() {
                                let (player_c, facing_direction_c, mut target_position_c): (
                                    specs::ReadStorage<Player>,
                                    specs::ReadStorage<FacingDirection>,
                                    specs::WriteStorage<TargetPosition>,
                                ) = game_state.world.system_data();

                                for (_, facing_direction, target_position) in
                                    (&player_c, &facing_direction_c, &mut target_position_c).join()
                                {
                                    // Help linter
                                    #[cfg(debug_assertions)]
                                    let facing_direction = facing_direction as &FacingDirection;
                                    #[cfg(debug_assertions)]
                                    let target_position = target_position as &mut TargetPosition;

                                    if !target_position.is_moving {
                                        let height = tile_map.tiles.len();
                                        let width = tile_map.tiles[0].len();

                                        if let Some((dx, dy)) = match facing_direction.direction {
                                            GameDirection::Down => {
                                                if target_position.y < height - 1 {
                                                    Some((0, 1))
                                                } else {
                                                    None
                                                }
                                            }
                                            GameDirection::Right => {
                                                if target_position.x < width - 1 {
                                                    Some((1, 0))
                                                } else {
                                                    None
                                                }
                                            }
                                            GameDirection::Up => {
                                                if target_position.y > 0 {
                                                    Some((0, -1))
                                                } else {
                                                    None
                                                }
                                            }
                                            GameDirection::Left => {
                                                if target_position.x > 0 {
                                                    Some((-1, 0))
                                                } else {
                                                    None
                                                }
                                            }
                                        } {
                                            let x =
                                                (target_position.x as isize + dx).max(0) as usize;
                                            let y =
                                                (target_position.y as isize + dy).max(0) as usize;

                                            if let Some(target_entity) =
                                                tile_map.get_tile(x, y).entity
                                            {
                                                // TODO: if entity has a spritesheet and/or a facing direction, then turn entity to face player

                                                // TODO: get interactable component from entity which can define how they should interact

                                                if let Some(interactable) = game_state
                                                    .world
                                                    .read_component::<Interactable>()
                                                    .get(target_entity)
                                                {
                                                    let player_entity = tile_map
                                                        .get_tile(
                                                            target_position.x,
                                                            target_position.y,
                                                        )
                                                        .entity
                                                        .expect(
                                                            "Player is not in target position?",
                                                        );

                                                    let handler = &interactable.handler;

                                                    if let Some(scene_builder) =
                                                        handler(player_entity, target_entity)
                                                    {
                                                        return Ok(Some(SceneSwitch::Push(
                                                            scene_builder,
                                                        )));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
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

    fn should_input_previous(&self) -> bool {
        true
    }

    fn should_update_previous(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        "OverworldScene"
    }
}

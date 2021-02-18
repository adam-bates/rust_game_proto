use super::{
    config,
    ecs::{
        components::{
            CurrentPosition, Door, Drawable, EntityName, FacingDirection, Id, Interactable,
            MapName, SpriteRow, SpriteSheet,
        },
        resources::{DoorRequest, PlayerMovementRequest, ShouldUpdateBackgroundTiles},
    },
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameDirection, GameInput},
    maps,
    save::SaveData,
    types::{Scene, SceneBuilder, SceneSwitch},
    TextBoxScene,
};
use specs::{Builder, Entity, WorldExt};
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

const TILE_MAP_DEFINITION_FILE: &str = "/bin/maps/varrock";

pub struct VarrockOverworldScene {
    scene_entities: Vec<Entity>,
}

impl VarrockOverworldScene {
    pub fn new(game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        {
            let mut door_request = game_state.world.fetch_mut::<DoorRequest>();
            door_request.requesting.take();
        }
        {
            let mut player_movement_request = game_state.world.fetch_mut::<PlayerMovementRequest>();
            player_movement_request.last_requested_direction = None;
            player_movement_request.last_requested_x_direction = None;
            player_movement_request.last_requested_y_direction = None;
            game_state.input_state.reset();
        }
        {
            let mut should_update_background_tiles =
                game_state.world.fetch_mut::<ShouldUpdateBackgroundTiles>();
            should_update_background_tiles.0 = true;
        }

        let save_data = {
            let save_data_r = game_state.world.try_fetch::<SaveData>().ok_or_else(|| {
                ggez::GameError::CustomError("SaveData resource not found".to_string())
            })?;

            (*save_data_r).clone()
        };

        let mut entities = HashMap::new();
        let mut scene_entities = vec![];

        let player_position = (save_data.player.position.x, save_data.player.position.y);
        let player_direction = save_data
            .player
            .position
            .facing
            .unwrap_or_else(|| GameDirection::Down);
        let player_entity =
            maps::find_and_move_player(game_state, player_position, player_direction)?;
        entities.insert(player_position, player_entity);

        let pallet_town_map = save_data
            .maps
            .get(&MapName::PalletTown)
            .expect("Pallet town data not in save file");

        if let Some(wise_old_man) = pallet_town_map
            .entity_instances
            .get(&EntityName::WiseOldMan)
        {
            let npc_position = (wise_old_man.position.x, wise_old_man.position.y);
            let npc_entity = game_state
                .world
                .create_entity()
                .with(Id::new("WiseOldMan"))
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
                        ggez::graphics::Color::from_rgb(20, 50, 150),
                    )?),
                    draw_params: ggez::graphics::DrawParam::default(),
                })
                .with(CurrentPosition {
                    x: npc_position.0 as f32,
                    y: npc_position.1 as f32,
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
                    direction: wise_old_man
                        .position
                        .facing
                        .unwrap_or_else(|| GameDirection::Down),
                })
                .with(Interactable {
                    handler: Box::new(|player_entity, target_entity| {
                        let scene_builder: SceneBuilder = Box::new(move |game_state, _| {
                            let scene = TextBoxScene::new(
                                game_state,
                                player_entity,
                                target_entity,
                                &format!("{:?} says hello to: {:?}", target_entity, player_entity),
                            );
                            Ok(Rc::new(RefCell::new(scene)))
                        });

                        Some(scene_builder)
                    }),
                })
                .build();
            entities.insert(npc_position, npc_entity);
            scene_entities.push(npc_entity);
        }

        let sign_1_position = (8, 6);
        let sign_1_entity = game_state
            .world
            .create_entity()
            .with(Id::new("Sign1"))
            .with(Interactable {
                handler: Box::new(|player_entity, target_entity| {
                    let scene_builder: SceneBuilder = Box::new(move |game_state, _| {
                        let scene = TextBoxScene::new(
                            game_state,
                            player_entity,
                            target_entity,
                            "And the sign says: Long haired freaky people need not apply.",
                        );
                        Ok(Rc::new(RefCell::new(scene)))
                    });

                    Some(scene_builder)
                }),
            })
            .build();
        entities.insert(sign_1_position, sign_1_entity);
        scene_entities.push(sign_1_entity);

        let door_1_position = (13, 0);
        let door_1_entity = game_state
            .world
            .create_entity()
            .with(Id::new("Door1"))
            .with(Door {
                id: 0,
                to_map: MapName::PalletTown,
                to_id: 0,
            })
            .build();
        entities.insert(door_1_position, door_1_entity);
        scene_entities.push(door_1_entity);

        maps::load_map(game_state, ctx, TILE_MAP_DEFINITION_FILE, &mut entities)?;

        let entities: Vec<Entity> = entities.values().cloned().collect();

        Ok(Self { scene_entities })
    }
}

impl std::fmt::Debug for VarrockOverworldScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {{ ... }}", self.name()))
    }
}

impl Scene for VarrockOverworldScene {
    fn dispose(&mut self, game_state: &mut GameState, _ctx: &mut ggez::Context) -> GameResult {
        maps::dispose_map(game_state, self.scene_entities.as_slice())
    }

    #[tracing::instrument]
    fn update(
        &mut self,
        _game_state: &mut GameState,
        _ctx: &mut ggez::Context,
        _delta_secs: f32,
    ) -> GameResult<Option<SceneSwitch>> {
        Ok(None)
    }

    #[tracing::instrument]
    fn draw(&self, game_state: &GameState, ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn input(
        &mut self,
        _game_state: &mut GameState,
        _ctx: &mut ggez::Context,
        _input: GameInput,
    ) -> GameResult<Option<SceneSwitch>> {
        Ok(None)
    }

    fn should_input_previous(&self) -> bool {
        true
    }

    fn should_update_previous(&self) -> bool {
        true
    }

    fn should_draw_previous(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        "VarrockOverworldScene"
    }
}

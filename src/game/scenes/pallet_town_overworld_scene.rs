use super::{
    config,
    ecs::{
        components::{CurrentPosition, Drawable, Player, TargetPosition},
        resources::{CameraBounds, Frame, Tile, TileMap},
    },
    error::types::GameResult,
    game_state::GameState,
    input::types::GameInput,
    types::{Scene, SceneSwitch},
};
use config::VIEWPORT_TILES_WIDTH_F32;
use specs::{Builder, Entity, Join, WorldExt};
use std::sync::Arc;

pub struct PalletTownOverworldScene;

fn find_and_move_player(game_state: &mut GameState, position: (usize, usize)) -> Entity {
    let (player_c, mut current_position_c, mut target_position_c): (
        specs::ReadStorage<Player>,
        specs::WriteStorage<CurrentPosition>,
        specs::WriteStorage<TargetPosition>,
    ) = game_state.world.system_data();

    let mut player_entity = None;
    for entity in player_c.fetched_entities().join() {
        player_entity = Some(entity);
    }
    let player_entity = player_entity.expect("No player entity in world");

    for (_, current_position, target_position) in
        (&player_c, &mut current_position_c, &mut target_position_c).join()
    {
        // Help linter
        #[cfg(debug_assertions)]
        let current_position = current_position as &mut CurrentPosition;
        #[cfg(debug_assertions)]
        let target_position = target_position as &mut TargetPosition;

        current_position.x = position.0 as f32;
        current_position.y = position.1 as f32;
        target_position.x = position.0;
        target_position.y = position.1;
    }

    player_entity
}

impl PalletTownOverworldScene {
    pub fn new(game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        let image = ggez::graphics::Image::new(ctx, "/pallet_town_spritesheet.png")?;

        let background_spritesheet_tile_width =
            image.width() as usize / config::TILE_PIXELS_SIZE_USIZE;
        let background_spritesheet_tile_height =
            image.height() as usize / config::TILE_PIXELS_SIZE_USIZE;

        let background = ggez::graphics::spritebatch::SpriteBatch::new(image);

        let background_width = 25;
        let background_height = 20;
        let tile_data = [
            2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 17, 16, 17,
            10, 41, 10, 41, 10, 41, 10, 41, 10, 41, 10, 41, 10, 41, 10, 41, 10, 41, 10, 41, 16, 17,
            2, 1, 25, 21, 15, 21, 21, 15, 15, 15, 21, 15, 21, 21, 15, 21, 21, 21, 15, 21, 21, 21,
            21, 26, 2, 17, 16, 32, 3, 4, 4, 5, 15, 15, 21, 15, 15, 21, 21, 21, 21, 21, 21, 15, 21,
            21, 21, 21, 31, 17, 2, 1, 25, 18, 19, 19, 20, 21, 36, 15, 15, 21, 15, 3, 4, 4, 4, 4, 4,
            4, 5, 21, 21, 26, 2, 17, 16, 32, 18, 19, 19, 20, 21, 21, 15, 15, 21, 21, 18, 19, 19,
            19, 19, 19, 19, 20, 21, 15, 31, 17, 2, 1, 25, 18, 19, 19, 20, 15, 15, 15, 21, 21, 21,
            33, 6, 6, 6, 7, 6, 6, 35, 15, 15, 26, 2, 17, 16, 32, 18, 19, 19, 20, 21, 21, 15, 15,
            15, 15, 21, 21, 21, 21, 15, 21, 15, 21, 21, 21, 31, 17, 2, 1, 25, 33, 34, 34, 35, 21,
            15, 15, 21, 21, 15, 15, 15, 15, 15, 15, 15, 21, 15, 21, 21, 26, 2, 17, 16, 32, 21, 21,
            3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 21, 21, 21, 31, 17, 2, 1, 25, 15, 21, 18,
            19, 19, 19, 12, 6, 6, 6, 6, 13, 14, 19, 19, 19, 20, 15, 21, 21, 26, 2, 17, 16, 32, 15,
            21, 18, 19, 19, 19, 27, 21, 15, 21, 15, 21, 29, 19, 19, 19, 20, 21, 21, 21, 31, 17, 2,
            1, 25, 21, 21, 18, 19, 19, 19, 27, 26, 25, 26, 25, 21, 29, 19, 19, 19, 20, 21, 21, 21,
            26, 2, 17, 16, 32, 15, 15, 18, 19, 19, 19, 27, 40, 11, 40, 11, 21, 29, 19, 19, 19, 20,
            21, 15, 21, 31, 17, 2, 1, 25, 21, 15, 18, 19, 19, 19, 27, 21, 36, 21, 21, 21, 29, 19,
            19, 19, 20, 15, 15, 21, 26, 2, 17, 16, 32, 15, 21, 18, 19, 19, 19, 42, 43, 43, 43, 43,
            43, 44, 19, 19, 19, 20, 15, 21, 15, 31, 17, 2, 1, 25, 21, 21, 18, 19, 19, 19, 19, 19,
            19, 19, 19, 19, 19, 19, 19, 19, 20, 21, 21, 21, 26, 2, 17, 16, 32, 21, 15, 33, 34, 34,
            34, 34, 34, 22, 23, 23, 23, 23, 23, 23, 24, 20, 21, 21, 21, 31, 17, 2, 1, 25, 3, 4, 4,
            4, 5, 15, 15, 21, 37, 38, 38, 38, 38, 38, 38, 39, 35, 15, 21, 21, 26, 2, 17, 16, 32,
            33, 22, 23, 24, 35, 15, 21, 15, 37, 38, 38, 38, 38, 38, 38, 39, 21, 21, 15, 21, 31, 17,
        ]
        .iter_mut()
        .map(|n| *n - 1)
        .collect();
        let overlay_data = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 51, 50, 51,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ]
        .iter_mut()
        .map(|n| if *n > 0 { Some(*n - 1) } else { None })
        .collect();

        game_state.world.insert(CameraBounds {
            min_x: 0.,
            min_y: 0.,
            max_x: background_width as f32 - config::VIEWPORT_TILES_WIDTH_F32,
            max_y: background_height as f32 - config::VIEWPORT_TILES_HEIGHT_F32,
        });

        let player_position = (0, 0);
        let player_entity = find_and_move_player(game_state, player_position);

        let npc_entity = game_state
            .world
            .create_entity()
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
            .with(CurrentPosition { x: 1., y: 1. })
            .build();

        game_state.world.insert(TileMap {
            tiles: build_tiles(player_entity, npc_entity),
            tile_indices: tile_data,
            overlay_indices: overlay_data,
            animation: vec![
                Frame {
                    idx: 0,
                    tile_ids: vec![37, 44],
                },
                Frame {
                    idx: 0,
                    tile_ids: vec![22, 29],
                },
            ],
            sprite_sheet_width: background_spritesheet_tile_width,
            sprite_sheet_height: background_spritesheet_tile_height,
            to_draw: vec![],
            overlay: background.clone(),
            background,
            background_param: ggez::graphics::DrawParam::default(),
        });

        Ok(Self)
    }
}

impl Scene for PalletTownOverworldScene {
    fn dispose(&mut self, game_state: &mut GameState, _ctx: &mut ggez::Context) -> GameResult {
        game_state.world.remove::<CameraBounds>();
        game_state.world.remove::<TileMap>();

        Ok(())
    }

    fn update(
        &mut self,
        _game_state: &mut GameState,
        _ctx: &mut ggez::Context,
        _delta_secs: f32,
    ) -> GameResult<Option<SceneSwitch>> {
        Ok(None)
    }

    fn draw(&self, game_state: &GameState, ctx: &mut ggez::Context) -> GameResult {
        let tile_map = game_state.world.read_resource::<TileMap>();

        use ggez::graphics::Drawable as GgezDrawable;
        tile_map.background.draw(ctx, tile_map.background_param)?;

        for drawable in &tile_map.to_draw {
            drawable.drawable.draw(ctx, drawable.draw_params)?;
        }

        tile_map.overlay.draw(ctx, tile_map.background_param)?;

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

    fn should_update_previous(&self) -> bool {
        true
    }
}

fn build_tiles(player_entity: Entity, npc_entity: Entity) -> Vec<Vec<Tile>> {
    vec![
        vec![
            Tile {
                entity: Some(player_entity),
                ..Default::default()
            },
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile {
                entity: Some(npc_entity),
                ..Default::default()
            },
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
        vec![
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
            Tile::default(),
        ],
    ]
}

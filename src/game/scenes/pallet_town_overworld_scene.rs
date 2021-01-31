use super::{
    config,
    ecs::{
        components::{CurrentPosition, Drawable, FacingDirection, SpriteRow, SpriteSheet},
        resources::{CameraBounds, TileMap},
    },
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameDirection, GameInput},
    maps::{find_and_move_player, TileMapDefinition},
    types::{Scene, SceneSwitch},
};
use specs::{Builder, WorldExt};
use std::{collections::HashMap, sync::Arc};

const TILE_MAP_DEFINITION_FILE: &str = "/bin/maps/pallet_town.bin";

pub struct PalletTownOverworldScene;

impl PalletTownOverworldScene {
    pub fn new(game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        // TODO: Take in player save information to derive player location, and any npc locations if already encountered

        let tile_map_definition = TileMapDefinition::load_from_file(ctx, TILE_MAP_DEFINITION_FILE)?;

        let mut entities = HashMap::new();

        let player_position = (7, 5);
        let player_entity = find_and_move_player(game_state, player_position);
        entities.insert(player_position, player_entity);

        let npc_position = (5, 5);
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
                direction: GameDirection::Down,
            })
            .build();
        entities.insert(npc_position, npc_entity);

        let tile_map_width = tile_map_definition.width;
        let tile_map_height = tile_map_definition.height;

        game_state.world.insert(CameraBounds {
            min_x: 0.,
            min_y: 0.,
            max_x: tile_map_width as f32 - config::VIEWPORT_TILES_WIDTH_F32,
            max_y: tile_map_height as f32 - config::VIEWPORT_TILES_HEIGHT_F32,
        });

        let tile_map = tile_map_definition.to_tile_map(ctx, &mut entities)?;

        game_state.world.insert(tile_map);

        Ok(Self)
    }
}

impl std::fmt::Debug for PalletTownOverworldScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {{ ... }}", self.name()))
    }
}

impl Scene for PalletTownOverworldScene {
    fn dispose(&mut self, game_state: &mut GameState, _ctx: &mut ggez::Context) -> GameResult {
        game_state.world.remove::<CameraBounds>();
        game_state.world.remove::<TileMap>();

        Ok(())
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

    fn should_update_previous(&self) -> bool {
        true
    }

    fn should_draw_previous(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        "PalletTownOverworldScene"
    }
}

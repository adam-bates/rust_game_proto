use super::{
    config,
    ecs::{
        components::{
            CurrentPosition, Drawable, FacingDirection, Player, SpriteRow, SpriteSheet,
            TargetPosition,
        },
        resources::Tile,
    },
    error::types::GameResult,
    game_state::GameState,
    input::types::GameDirection,
};
use serde::{Deserialize, Serialize};
use specs::{Builder, Entity, Join, WorldExt};
use std::{collections::HashMap, sync::Arc};

#[derive(Serialize, Deserialize, Debug)]
pub struct TileMapDefinition {
    pub width: usize,
    pub height: usize,
    pub player_x: usize,
    pub player_y: usize,
    pub sprite_sheet_filename: String,
    pub background_tile_ids: Vec<usize>,
    pub overlay_tile_ids: Vec<Option<usize>>,
    pub tiles: Vec<MapTile>,
}

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

impl TileMapDefinition {
    pub fn build_tiles(
        &self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
    ) -> GameResult<Vec<Vec<Tile>>> {
        let player_position = (
            self.player_x / config::TILE_PIXELS_SIZE_USIZE,
            self.player_y / config::TILE_PIXELS_SIZE_USIZE,
        );
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
            .with(CurrentPosition { x: 5., y: 5. })
            .with(SpriteSheet::new(vec![SpriteRow::new(1)]))
            .with(FacingDirection {
                direction: GameDirection::Down,
            })
            .build();

        let mut id_tile_types = HashMap::new();
        for map_tile in self.tiles.iter() {
            id_tile_types.insert(map_tile.id, &map_tile.tile_type);
        }

        let mut x_y_tiles = vec![];

        for y in 0..self.height {
            let mut x_tiles = vec![];

            for x in 0..self.width {
                let entity = if x == player_position.0 && y == player_position.1 {
                    Some(player_entity)
                } else if x == 5 && y == 5 {
                    Some(npc_entity)
                } else {
                    None
                };

                x_tiles.push(Tile {
                    entity,
                    tile_type: id_tile_types
                        .get(&self.background_tile_ids[y * self.width + x])
                        .map(|v| *v)
                        .cloned(),
                    overlay: None,
                })
            }

            x_y_tiles.push(x_tiles);
        }

        Ok(x_y_tiles)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TileType {
    Wall,
    Water,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapTile {
    pub id: usize,
    pub tile_type: TileType,
    pub animation: Option<Vec<MapTileAnimationFrame>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapTileAnimationFrame {
    pub tile_id: usize,
}

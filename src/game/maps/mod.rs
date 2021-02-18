use super::{
    config,
    ecs::{
        components::{CurrentPosition, FacingDirection, MapName, Player, TargetPosition},
        resources::{CameraBounds, Frame, Tile, TileMap},
    },
    error::types::GameResult,
    game_state::GameState,
    input::types::GameDirection,
};
use serde::{Deserialize, Serialize};
use specs::{Entity, Join, WorldExt};
use std::collections::HashMap;

pub fn load_map(
    game_state: &mut GameState,
    ctx: &mut ggez::Context,
    map_file_path: &str,
    entities: &mut HashMap<(usize, usize), Entity>,
) -> GameResult {
    let tile_map_definition = TileMapDefinition::load_from_file(ctx, map_file_path)?;

    let tile_map_width = tile_map_definition.width;
    let tile_map_height = tile_map_definition.height;

    game_state.world.insert(CameraBounds {
        min_x: 0.,
        min_y: 0.,
        max_x: tile_map_width as f32 - config::VIEWPORT_TILES_WIDTH_F32,
        max_y: tile_map_height as f32 - config::VIEWPORT_TILES_HEIGHT_F32,
    });

    let tile_map = tile_map_definition.to_tile_map(ctx, entities)?;

    game_state.world.insert(tile_map);

    Ok(())
}

pub fn dispose_map(game_state: &mut GameState, entities: &[Entity]) -> GameResult {
    game_state.world.remove::<CameraBounds>();
    game_state.world.remove::<TileMap>();

    if let Err(e) = game_state.world.delete_entities(entities) {
        return Err(ggez::GameError::CustomError(format!(
            "Wrong generation error when deleting entities in OverworldScene::dispose: {}",
            e
        )));
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TileMapDefinition {
    pub width: usize,
    pub height: usize,
    pub background: TileLayer,
    pub overlay: TileLayer,
}

fn build_spritesheet_from_layer(
    layer: &TileLayer,
    ctx: &mut ggez::Context,
) -> GameResult<(ggez::graphics::spritebatch::SpriteBatch, usize, usize)> {
    let loaded_images: Vec<GameResult<ggez::graphics::Image>> = layer
        .tile_sets
        .iter()
        .map(|tile_set| ggez::graphics::Image::new(ctx, tile_set.sprite_sheet_filename.to_string()))
        .collect();

    if let Some(Err(e)) = loaded_images
        .iter()
        .find(|loaded_image| loaded_image.is_err())
    {
        return Err(e.clone());
    }

    let loaded_images: Vec<ggez::graphics::Image> = loaded_images
        .into_iter()
        .map(|loaded_image| loaded_image.unwrap())
        .collect();

    let canvas_width: u16 = loaded_images
        .iter()
        .map(|image| image.width())
        .max()
        .expect("No spritesheets loaded?");

    let canvas_height: u16 = loaded_images
        .iter()
        .map(|image| image.height())
        .sum::<u16>();

    let canvas = ggez::graphics::Canvas::new(
        ctx,
        canvas_width,
        canvas_height,
        ggez::conf::NumSamples::One,
        ggez::graphics::get_window_color_format(ctx),
    )?;

    let screen_coords = ggez::graphics::screen_coordinates(ctx);

    ggez::graphics::set_canvas(ctx, Some(&canvas));
    ggez::graphics::set_screen_coordinates(
        ctx,
        [0., 0., canvas_width as f32, canvas_height as f32].into(),
    )?;

    let mut y_offset = 0.;

    if let Some(Err(e)) = loaded_images
        .iter()
        .map(|image| {
            ggez::graphics::draw(
                ctx,
                image,
                // Canvas images are drawn upside down
                ggez::graphics::DrawParam::default()
                    .scale([1., -1.])
                    .dest([0., y_offset + canvas_height as f32]),
            )?;
            y_offset -= image.height() as f32;

            Ok(())
        })
        .find(|res: &GameResult| res.is_err())
    {
        return Err(e);
    }

    ggez::graphics::set_canvas(ctx, None);
    ggez::graphics::set_screen_coordinates(ctx, screen_coords)?;

    let spritesheet_image = canvas.into_inner();

    let width = spritesheet_image.width() as usize / config::TILE_PIXELS_SIZE_USIZE;
    let height = spritesheet_image.height() as usize / config::TILE_PIXELS_SIZE_USIZE;

    let spritesheet = ggez::graphics::spritebatch::SpriteBatch::new(spritesheet_image);

    Ok((spritesheet, width, height))
}

fn build_animation_from_layer(layer: TileLayer) -> Vec<Frame> {
    layer
        .tile_sets
        .into_iter()
        .flat_map(|set| set.tiles)
        .filter(|t| t.animation.is_some())
        .map(|t| {
            let tile_ids: Vec<usize> = t
                .animation
                .unwrap()
                .iter()
                .map(|frame| frame.tile_id)
                .collect();

            Frame { idx: 0, tile_ids }
        })
        .collect()
}

pub fn find_and_move_player(
    game_state: &mut GameState,
    position: (usize, usize),
    direction: GameDirection,
) -> GameResult<Entity> {
    let (player_c, mut current_position_c, mut target_position_c, mut facing_direction_c): (
        specs::ReadStorage<Player>,
        specs::WriteStorage<CurrentPosition>,
        specs::WriteStorage<TargetPosition>,
        specs::WriteStorage<FacingDirection>,
    ) = game_state.world.system_data();

    let mut player_entity = None;
    for entity in player_c.fetched_entities().join() {
        player_entity = Some(entity);
    }
    let player_entity = player_entity
        .ok_or_else(|| ggez::GameError::CustomError("No player entity in world".to_string()))?;

    for (_, current_position, target_position, facing_direction) in (
        &player_c,
        &mut current_position_c,
        &mut target_position_c,
        &mut facing_direction_c,
    )
        .join()
    {
        // Help linter
        #[cfg(debug_assertions)]
        let current_position = current_position as &mut CurrentPosition;
        #[cfg(debug_assertions)]
        let target_position = target_position as &mut TargetPosition;
        #[cfg(debug_assertions)]
        let facing_direction = facing_direction as &mut FacingDirection;

        current_position.x = position.0 as f32;
        current_position.y = position.1 as f32;
        target_position.from_x = position.0;
        target_position.from_y = position.1;
        target_position.x = position.0;
        target_position.y = position.1;
        facing_direction.direction = direction;
    }

    Ok(player_entity)
}

impl TileMapDefinition {
    pub fn load_from_file(ctx: &mut ggez::Context, filename: &str) -> GameResult<Self> {
        let file = ctx
            .filesystem
            .find_vfs(&ctx.filesystem.assets_path)
            .ok_or_else(|| {
                ggez::GameError::FilesystemError("Couldn't find asset filesystem:".to_string())
            })?
            .open(&std::path::PathBuf::from(filename))?;

        bincode::deserialize_from(file).or_else(|e| {
            Err(ggez::GameError::ResourceLoadError(format!(
                "Couldn't load map binary: {}",
                e
            )))
        })
    }

    pub fn to_tile_map(
        self,
        ctx: &mut ggez::Context,
        entities: &mut HashMap<(usize, usize), Entity>,
    ) -> GameResult<TileMap> {
        let (background_spritesheet, background_width, background_height) =
            build_spritesheet_from_layer(&self.background, ctx)?;

        let (overlay_spritesheet, overlay_width, overlay_height) =
            build_spritesheet_from_layer(&self.overlay, ctx)?;

        let tiles = self.build_tiles(entities)?;

        let background_indices = self.background.tile_ids.clone();
        let overlay_indices = self.overlay.tile_ids.clone();

        let background_animation = build_animation_from_layer(self.background);
        let overlay_animation = build_animation_from_layer(self.overlay);

        Ok(TileMap {
            tiles,
            background_indices,
            overlay_indices,
            background_animation,
            overlay_animation,
            background: background_spritesheet,
            background_width,
            background_height,
            overlay: overlay_spritesheet,
            overlay_width,
            overlay_height,
            spritesheet_param: ggez::graphics::DrawParam::default(),
            to_draw: vec![],
        })
    }

    pub fn build_tiles(
        &self,
        entities: &mut HashMap<(usize, usize), Entity>,
    ) -> GameResult<Vec<Vec<Tile>>> {
        let id_tile_types =
            self.background
                .tile_sets
                .iter()
                .fold(HashMap::new(), |mut map, tile_set| {
                    tile_set.tiles.iter().for_each(|map_tile| {
                        map.insert(map_tile.id, map_tile.tile_type.clone());
                    });

                    map
                });

        let mut x_y_tiles = vec![];

        for y in 0..self.height {
            let mut x_tiles = vec![];

            for x in 0..self.width {
                let entity = entities.remove(&(x, y));

                x_tiles.push(Tile {
                    entity,
                    tile_type: self.background.tile_ids[y * self.width + x]
                        .and_then(|tile_id| id_tile_types.get(&tile_id).cloned()),
                })
            }

            x_y_tiles.push(x_tiles);
        }

        Ok(x_y_tiles)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TileLayer {
    pub tile_ids: Vec<Option<usize>>,
    pub tile_sets: Vec<TileSet>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TileSet {
    pub sprite_sheet_filename: String,
    pub tiles: Vec<MapTile>,
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

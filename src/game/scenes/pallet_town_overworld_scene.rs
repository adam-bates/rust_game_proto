use super::{
    config,
    ecs::resources::{CameraBounds, Frame, TileMap},
    error::types::GameResult,
    game_state::GameState,
    input::types::GameInput,
    maps::TileMapDefinition,
    types::{Scene, SceneSwitch},
};
use specs::WorldExt;

pub struct PalletTownOverworldScene;

impl PalletTownOverworldScene {
    pub fn new(game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        let file = ctx
            .filesystem
            .find_vfs(&ctx.filesystem.assets_path)
            .unwrap()
            .open(&std::path::PathBuf::from("/maps/pallet_town_tile_map.bin"))?;

        let tile_map_definition: TileMapDefinition =
            bincode::deserialize_from(file).or_else(|e| {
                Err(ggez::GameError::ResourceLoadError(format!(
                    "Couldn't load map binary: {}",
                    e
                )))
            })?;

        let image = ggez::graphics::Image::new(
            ctx,
            format!(
                "/spritesheets/{}",
                tile_map_definition.sprite_sheet_filename
            ),
        )?;

        let background_spritesheet_tile_width =
            image.width() as usize / config::TILE_PIXELS_SIZE_USIZE;
        let background_spritesheet_tile_height =
            image.height() as usize / config::TILE_PIXELS_SIZE_USIZE;

        let background = ggez::graphics::spritebatch::SpriteBatch::new(image);

        let tiles = tile_map_definition.build_tiles(game_state, ctx)?;

        let background_width = tile_map_definition.width;
        let background_height = tile_map_definition.height;
        let tile_data = tile_map_definition.background_tile_ids;
        let overlay_data = tile_map_definition.overlay_tile_ids;

        game_state.world.insert(CameraBounds {
            min_x: 0.,
            min_y: 0.,
            max_x: background_width as f32 - config::VIEWPORT_TILES_WIDTH_F32,
            max_y: background_height as f32 - config::VIEWPORT_TILES_HEIGHT_F32,
        });

        let animation: Vec<Frame> = tile_map_definition
            .tiles
            .into_iter()
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
            .collect();

        game_state.world.insert(TileMap {
            tiles,
            tile_indices: tile_data,
            overlay_indices: overlay_data,
            animation,
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
        use ggez::graphics::Drawable;
        let tile_map = game_state.world.read_resource::<TileMap>();

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

    fn name(&self) -> &str {
        "PalletTownOverworldScene"
    }
}

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

const TILE_MAP_DEFINITION_FILE: &str = "/bin/area/pallet_town_tile_map.bin";

pub struct PalletTownOverworldScene;

impl PalletTownOverworldScene {
    pub fn new(game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        let tile_map_definition = TileMapDefinition::load_from_file(ctx, TILE_MAP_DEFINITION_FILE)?;

        println!("{:#?}", tile_map_definition);

        let loaded_background_images: Vec<GameResult<ggez::graphics::Image>> = tile_map_definition
            .background
            .tile_sets
            .iter()
            .map(|tile_set| {
                ggez::graphics::Image::new(
                    ctx,
                    format!("/spritesheets/{}", tile_set.sprite_sheet_filename),
                )
            })
            .collect();

        if let Some(Err(e)) = loaded_background_images
            .iter()
            .find(|loaded_image| loaded_image.is_err())
        {
            return Err(e.clone());
        }

        let loaded_background_images: Vec<ggez::graphics::Image> = loaded_background_images
            .into_iter()
            .map(|loaded_image| loaded_image.unwrap())
            .collect();

        let background_canvas_width: u16 = loaded_background_images
            .iter()
            .map(|image| image.width())
            .max()
            .expect("No spritesheets loaded?");

        let background_canvas_height: u16 = loaded_background_images
            .iter()
            .map(|image| image.height())
            .sum::<u16>();

        let background_canvas = ggez::graphics::Canvas::new(
            ctx,
            background_canvas_width,
            background_canvas_height,
            ggez::conf::NumSamples::One,
            ggez::graphics::get_window_color_format(ctx),
        )?;

        let screen_coords = ggez::graphics::screen_coordinates(ctx);

        ggez::graphics::set_canvas(ctx, Some(&background_canvas));
        ggez::graphics::set_screen_coordinates(
            ctx,
            [
                0.,
                0.,
                background_canvas_width as f32,
                background_canvas_height as f32,
            ]
            .into(),
        )?;

        let mut y_offset = 0.;

        if let Some(Err(e)) = loaded_background_images
            .iter()
            .map(|image| {
                ggez::graphics::draw(
                    ctx,
                    image,
                    // Canvas images are drawn upside down
                    ggez::graphics::DrawParam::default()
                        .scale([1., -1.])
                        .dest([0., y_offset + background_canvas_height as f32]),
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

        // REPEAT FOR OVERLAY
        // TODO: Functions

        let loaded_overlay_images: Vec<GameResult<ggez::graphics::Image>> = tile_map_definition
            .overlay
            .tile_sets
            .iter()
            .map(|tile_set| {
                ggez::graphics::Image::new(
                    ctx,
                    format!("/spritesheets/{}", tile_set.sprite_sheet_filename),
                )
            })
            .collect();

        if let Some(Err(e)) = loaded_overlay_images
            .iter()
            .find(|loaded_image| loaded_image.is_err())
        {
            return Err(e.clone());
        }

        let loaded_overlay_images: Vec<ggez::graphics::Image> = loaded_overlay_images
            .into_iter()
            .map(|loaded_image| loaded_image.unwrap())
            .collect();

        let overlay_canvas_width: u16 = loaded_overlay_images
            .iter()
            .map(|image| image.width())
            .max()
            .expect("No spritesheets loaded?");

        let overlay_canvas_height: u16 = loaded_overlay_images
            .iter()
            .map(|image| image.height())
            .sum::<u16>();

        let overlay_canvas = ggez::graphics::Canvas::new(
            ctx,
            overlay_canvas_width,
            overlay_canvas_height,
            ggez::conf::NumSamples::One,
            ggez::graphics::get_window_color_format(ctx),
        )?;

        let screen_coords = ggez::graphics::screen_coordinates(ctx);

        ggez::graphics::set_canvas(ctx, Some(&overlay_canvas));
        ggez::graphics::set_screen_coordinates(
            ctx,
            [
                0.,
                0.,
                overlay_canvas_width as f32,
                overlay_canvas_height as f32,
            ]
            .into(),
        )?;

        let mut y_offset = 0.;

        if let Some(Err(e)) = loaded_overlay_images
            .iter()
            .map(|image| {
                ggez::graphics::draw(
                    ctx,
                    image,
                    // Canvas images are drawn upside down
                    ggez::graphics::DrawParam::default()
                        .scale([1., -1.])
                        .dest([0., y_offset + overlay_canvas_height as f32]),
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

        let background_sprite_sheets_image = background_canvas.into_inner();

        let background_width =
            background_sprite_sheets_image.width() as usize / config::TILE_PIXELS_SIZE_USIZE;
        let background_height =
            background_sprite_sheets_image.height() as usize / config::TILE_PIXELS_SIZE_USIZE;

        let background_sprite_sheets_batch =
            ggez::graphics::spritebatch::SpriteBatch::new(background_sprite_sheets_image);

        let overlay_sprite_sheets_image = overlay_canvas.into_inner();

        let overlay_width =
            overlay_sprite_sheets_image.width() as usize / config::TILE_PIXELS_SIZE_USIZE;
        let overlay_height =
            overlay_sprite_sheets_image.height() as usize / config::TILE_PIXELS_SIZE_USIZE;

        let overlay_sprite_sheets_batch =
            ggez::graphics::spritebatch::SpriteBatch::new(overlay_sprite_sheets_image);

        let tiles = tile_map_definition.build_tiles(game_state, ctx)?;

        let tile_map_width = tile_map_definition.width;
        let tile_map_height = tile_map_definition.height;
        let tile_data = tile_map_definition.background.tile_ids;
        let overlay_data = tile_map_definition.overlay.tile_ids;

        game_state.world.insert(CameraBounds {
            min_x: 0.,
            min_y: 0.,
            max_x: tile_map_width as f32 - config::VIEWPORT_TILES_WIDTH_F32,
            max_y: tile_map_height as f32 - config::VIEWPORT_TILES_HEIGHT_F32,
        });

        let background_animation: Vec<Frame> = tile_map_definition
            .background
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
            .collect();

        let overlay_animation: Vec<Frame> = tile_map_definition
            .overlay
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
            .collect();

        game_state.world.insert(TileMap {
            tiles,
            tile_indices: tile_data,
            overlay_indices: overlay_data,
            background_animation,
            overlay_animation,
            to_draw: vec![],
            overlay: overlay_sprite_sheets_batch,
            overlay_width,
            overlay_height,
            background: background_sprite_sheets_batch,
            background_width,
            background_height,
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

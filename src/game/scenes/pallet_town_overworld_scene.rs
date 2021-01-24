use super::{
    config,
    ecs::{
        components::{CurrentPosition, Drawable, Player, TargetPosition},
        resources::{Camera, CameraBounds, Tile, TileMap},
    },
    error::types::GameResult,
    game_state::GameState,
    input::types::GameInput,
    types::{Scene, SceneSwitch},
};
use specs::{Builder, Entity, Join, WorldExt};

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
        let image = ggez::graphics::Image::new(ctx, "/background_pallet_town.png")?;

        let background_width = image.width() as usize / config::TILE_PIXELS_SIZE_USIZE;
        let background_height = image.height() as usize / config::TILE_PIXELS_SIZE_USIZE;

        let mut background = ggez::graphics::spritebatch::SpriteBatch::new(image);

        let inverse_background_width = 1. / background_width as f32;
        let inverse_background_height = 1. / background_height as f32;

        for x in 0..config::VIEWPORT_PIXELS_WIDTH_USIZE {
            for y in 0..config::VIEWPORT_PIXELS_HEIGHT_USIZE {
                background.add(
                    ggez::graphics::DrawParam::default()
                        .src(
                            [
                                x as f32 * inverse_background_width,
                                y as f32 * inverse_background_height,
                                inverse_background_width,
                                inverse_background_height,
                            ]
                            .into(),
                        )
                        .dest([
                            x as f32 * config::TILE_PIXELS_SIZE_F32,
                            y as f32 * config::TILE_PIXELS_SIZE_F32,
                        ]),
                );
            }
        }

        game_state.world.insert(CameraBounds {
            min_x: 0.,
            min_y: 0.,
            max_x: 5.,
            max_y: 9.,
        });

        let player_position = (0, 0);
        let player_entity = find_and_move_player(game_state, player_position);

        let npc_entity = game_state
            .world
            .create_entity()
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
                    ggez::graphics::Color::from_rgb(20, 50, 150),
                )?),
                draw_params: ggez::graphics::DrawParam::default(),
            })
            .with(CurrentPosition { x: 1., y: 1. })
            .build();

        game_state.world.insert(TileMap {
            tiles: build_tiles(player_entity, npc_entity),
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
        let camera = game_state.world.read_resource::<Camera>();
        let tile_map = game_state.world.read_resource::<TileMap>();

        use ggez::graphics::Drawable as GgezDrawable;
        tile_map.background.draw(ctx, tile_map.background_param)?;

        let mut drawable_c = game_state.world.write_component::<Drawable>();

        // Draw in order of y first to emulate z-axis
        for y in camera.top..camera.bottom {
            for x in camera.left..camera.right {
                if let Some(entity) = tile_map.get_tile(x, y).entity {
                    if let Some(drawable) = drawable_c.get_mut(entity) {
                        drawable.drawable.draw(ctx, drawable.draw_params)?;
                    }
                }
            }
        }

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
        ],
    ]
}

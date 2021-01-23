use super::{
    config,
    ecs::{
        components::{CurrentPosition, Drawable, Player, TargetPosition},
        resources::{Camera, CameraBounds, Tile, TileMap, TileType},
    },
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameDirection, GameInput},
    types::{Scene, SceneSwitch},
};
use config::{TILE_PIXELS_SIZE_USIZE, VIEWPORT_TILES_HEIGHT_USIZE, VIEWPORT_TILES_WIDTH_USIZE};
use specs::{Builder, Entity, Join, WorldExt};

pub struct PalletTownOverworldScene {
    background: ggez::graphics::spritebatch::SpriteBatch,
    background_param: ggez::graphics::DrawParam,
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

impl PalletTownOverworldScene {
    pub fn new(game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        let image = ggez::graphics::Image::new(ctx, "/background_pallet_town.png")?;

        let background_width = image.width() as usize / config::TILE_PIXELS_SIZE_USIZE;
        let background_height = image.height() as usize / config::TILE_PIXELS_SIZE_USIZE;

        let mut background = ggez::graphics::spritebatch::SpriteBatch::new(image);

        let inverse_background_width = 1. / background_width as f32;
        let inverse_background_height = 1. / background_height as f32;

        // let camera_width = config::VIEWPORT_TILES_WIDTH_USIZE as i32;
        // let camera_height = config::VIEWPORT_TILES_HEIGHT_USIZE as i32;

        // let pos_x = background_width as i32 - camera_width;
        // let pos_y = background_height as i32 - camera_height;

        for x in 0..background_width {
            for y in 0..background_height {
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
        });

        Ok(Self {
            background,
            background_param: ggez::graphics::DrawParam::default(),
        })
    }

    fn move_background_to(&mut self, target_position: &TargetPosition) {
        self.background.clear();

        // TODO
    }
}

impl Scene for PalletTownOverworldScene {
    fn dispose(&mut self, game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult {
        game_state.world.remove::<CameraBounds>();
        game_state.world.remove::<TileMap>();

        Ok(())
    }

    fn update(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        delta_secs: f32,
    ) -> GameResult<Option<SceneSwitch>> {
        let camera = game_state.world.read_resource::<Camera>();

        let tile_map = game_state.world.read_resource::<TileMap>();
        let (max_x, max_y) = tile_map.dimensions();

        let inverse_background_width = 1. / max_x as f32;
        let inverse_background_height = 1. / max_y as f32;

        let left = (camera.x as isize - 1).max(0) as usize;
        let right = (camera.x as usize + config::VIEWPORT_TILES_WIDTH_USIZE + 1).min(max_x);
        let top = (camera.y as isize - 1).max(0) as usize;
        let bottom = (camera.y as usize + config::VIEWPORT_TILES_HEIGHT_USIZE + 1).min(max_y);

        self.background.clear();

        for y in top..bottom {
            for x in left..right {
                self.background.add(
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

        Ok(None)
    }

    fn draw(&self, game_state: &GameState, ctx: &mut ggez::Context) -> GameResult {
        let camera = game_state.world.read_resource::<Camera>();

        use ggez::graphics::Drawable as GgezDrawable;
        let mut background_param = self.background_param.clone();
        if let ggez::graphics::Transform::Values { ref mut dest, .. } = background_param.trans {
            dest.x -= camera.x * config::TILE_PIXELS_SIZE_F32;
            dest.y -= camera.y * config::TILE_PIXELS_SIZE_F32;
        }
        self.background.draw(ctx, background_param)?;

        let tile_map = game_state.world.read_resource::<TileMap>();
        let (max_x, max_y) = tile_map.dimensions();

        let mut drawable_c = game_state.world.write_component::<Drawable>();

        let left = (camera.x as isize - 1).max(0) as usize;
        let right = (camera.x as usize + config::VIEWPORT_TILES_WIDTH_USIZE + 1).min(max_x);
        let top = (camera.y as isize - 1).max(0) as usize;
        let bottom = (camera.y as usize + config::VIEWPORT_TILES_HEIGHT_USIZE + 1).min(max_y);

        // Draw in order of y first to emulate z-axis
        for y in top..bottom {
            for x in left..right {
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
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        input: GameInput,
    ) -> GameResult<Option<SceneSwitch>> {
        // If pressed down direction:
        //  - move player's target_position
        //  - clear sprite_batch
        //  - add draw params from player's target_position

        /*
        In input handler:

                let player = game_state.world.read_storage::<Player>();
                let target_position = game_state.world.read_storage::<TargetPosition>();

                for (_, target_position) in (&player, &target_position).join() {
                    self.move_background_to(target_position);
                    break;
                }

        Don't add here though ... we should store the input as requested,
        then have a system with a looping timer running to check for input and handle the "current state" when the timer finishes
        */

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

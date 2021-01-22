use super::{
    config,
    ecs::{
        components::{Player, CurrentPosition, TargetPosition},
        resources::{Camera, CameraBounds},
        systems::{FollowPlayerSystem, PrintSystem},
    },
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameDirection, GameInput},
    types::{Scene, SceneSwitch},
};
use specs::{Builder, Join, WorldExt};

pub struct PalletTownOverworldScene {
    dispatcher: specs::Dispatcher<'static, 'static>,
    background: ggez::graphics::spritebatch::SpriteBatch,
    background_param: ggez::graphics::DrawParam,
}

impl PalletTownOverworldScene {
    pub fn new(game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        let image = ggez::graphics::Image::new(ctx, "/background_pallet_town.png")?;

        let background_width = image.width() as f32 / config::TILE_PIXELS_SIZE_F32;
        let background_height = image.height() as f32 / config::TILE_PIXELS_SIZE_F32;

        let mut background = ggez::graphics::spritebatch::SpriteBatch::new(image);

        let inverse_background_width = 1. / background_width;
        let inverse_background_height = 1. / background_height;

        let camera_width = config::VIEWPORT_TILES_WIDTH_USIZE as i32;
        let camera_height = config::VIEWPORT_TILES_HEIGHT_USIZE as i32;

        let pos_x = background_width as i32 - camera_width;
        let pos_y = background_height as i32 - camera_height;

        for x in pos_x..camera_width + pos_x {
            for y in pos_y..camera_height + pos_y {
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
                            (x - pos_x) as f32 * config::TILE_PIXELS_SIZE_F32,
                            (y - pos_y) as f32 * config::TILE_PIXELS_SIZE_F32,
                        ]),
                );
            }
        }

        // use ggez::graphics::Drawable;
        // sprite_batch.draw(ctx, ggez::graphics::DrawParam::default())?;

        let dispatcher = specs::DispatcherBuilder::new().build();

        Ok(Self {
            dispatcher,
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
    fn update(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>> {
        // println!("PalletTownOverworldScene::update");
        self.dispatcher.dispatch(&game_state.world);
        Ok(None)
    }

    fn draw(&self, game_state: &GameState, ctx: &mut ggez::Context) -> GameResult {
        use ggez::graphics::Drawable;
        self.background.draw(ctx, self.background_param)?;
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

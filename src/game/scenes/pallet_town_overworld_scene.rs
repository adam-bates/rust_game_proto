use super::{
    ecs::{
        components::{Player, RealPosition, TargetPosition},
        resources::{Camera, CameraBounds},
        systems::{FollowPlayerSystem, PrintSystem},
    },
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameButton, GameInput},
    types::{Scene, SceneSwitch},
};
use specs::{Builder, WorldExt};

pub struct PalletTownOverworldScene {
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl PalletTownOverworldScene {
    pub fn new(game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        let dispatcher = specs::DispatcherBuilder::new().build();

        Ok(Self { dispatcher })
    }
}

impl Scene for PalletTownOverworldScene {
    fn update(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>> {
        println!("PalletTownOverworldScene::update");
        self.dispatcher.dispatch(&game_state.world);
        Ok(None)
    }

    fn draw(&self, game_state: &GameState, ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn input(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        input: GameInput,
    ) -> GameResult<Option<SceneSwitch>> {
        Ok(None)
    }

    fn should_update_previous(&self) -> bool {
        true
    }
}

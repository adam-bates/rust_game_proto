use super::{
    ecs::resources::DeltaTime,
    error::types::GameResult,
    game_state::GameState,
    input::types::{GameButton, GameInput},
    save::{self, MetaSaveData, SaveSlot},
    types::{Scene, SceneBuilder, SceneSwitch},
    world, MainMenuScene, OverworldScene,
};
use std::{cell::RefCell, rc::Rc};

pub struct InGameScene;

impl InGameScene {
    pub fn new(
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        save_slot: SaveSlot,
        meta_data: MetaSaveData,
    ) -> GameResult<Self> {
        game_state.world = world::create_world();
        game_state.world.insert(DeltaTime::default());

        game_state.world.insert(save_slot);
        game_state.world.insert(meta_data);

        save::load(game_state, ctx, save_slot)?;

        Ok(Self)
    }
}

impl std::fmt::Debug for InGameScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {{ ... }}", self.name()))
    }
}

impl Scene for InGameScene {
    fn dispose(&mut self, game_state: &mut GameState, _ctx: &mut ggez::Context) -> GameResult {
        game_state.world.remove::<DeltaTime>();
        game_state.world.remove::<SaveSlot>();
        game_state.world.remove::<MetaSaveData>();
        Ok(())
    }

    fn on_create(
        &mut self,
        _game_state: &mut GameState,
        _ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>> {
        let scene_builder: SceneBuilder = Box::new(|game_state, ctx| {
            let scene = OverworldScene::new(game_state, ctx)?;
            Ok(Rc::new(RefCell::new(scene)))
        });

        Ok(Some(SceneSwitch::Push(scene_builder)))
    }

    #[tracing::instrument]
    fn update(
        &mut self,
        game_state: &mut GameState,
        _ctx: &mut ggez::Context,
        delta_secs: f32,
    ) -> GameResult<Option<SceneSwitch>> {
        if let Some(mut delta) = game_state.world.get_mut::<DeltaTime>() {
            delta.secs = delta_secs;
        }

        Ok(None)
    }

    #[tracing::instrument]
    fn draw(&self, _game_state: &GameState, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn input(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        input: GameInput,
    ) -> GameResult<Option<SceneSwitch>> {
        match input {
            GameInput::Button { button, pressed } => {
                if pressed {
                    match button {
                        GameButton::Select => {
                            let save_slot = *game_state.world.fetch::<SaveSlot>();
                            save::save(game_state, ctx, save_slot)?;
                            println!("Saved to slot: {}", save_slot.id());
                        }
                        GameButton::Secondary => {
                            let scene_builder: SceneBuilder = Box::new(|game_state, ctx| {
                                let scene = MainMenuScene::new(game_state, ctx)?;

                                Ok(Rc::new(RefCell::new(scene)))
                            });

                            return Ok(Some(SceneSwitch::ReplaceAll(scene_builder)));
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        Ok(None)
    }

    fn name(&self) -> &str {
        "InGameScene"
    }
}

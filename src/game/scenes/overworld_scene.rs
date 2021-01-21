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

pub struct OverworldScene {
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl OverworldScene {
    pub fn new(game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult<Self> {
        // TODO: Build from loaded save file

        let player_target_position = TargetPosition { x: 5., y: 7. };
        let player_real_position = RealPosition {
            x: player_target_position.x,
            y: player_target_position.y,
        };

        let camera = Camera {
            x: player_target_position.x,
            y: player_target_position.y,
        };

        game_state.world.insert(camera);

        let mut dispatcher = specs::DispatcherBuilder::new()
            .with(FollowPlayerSystem, "follow_player_system", &[])
            .with(PrintSystem, "print_system", &["follow_player_system"])
            .build();
        dispatcher.setup(&mut game_state.world);

        game_state
            .world
            .create_entity()
            .with(Player)
            .with(player_real_position)
            .with(player_target_position)
            .build();

        Ok(Self { dispatcher })
    }

    // TODO: Function to build from save file given a filesystem
}

impl Scene for OverworldScene {
    fn update(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>> {
        println!("OverworldScene::update");
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
        match input {
            GameInput::Button { button, pressed } => {
                if pressed {
                    match button {
                        GameButton::Start => game_state.world.insert(CameraBounds {
                            min_x: 0.,
                            min_y: 0.,
                            max_x: 2.,
                            max_y: 3.,
                        }),
                        GameButton::Select => {
                            game_state.world.remove::<CameraBounds>();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        Ok(None)
    }
}

use super::{
    error::types::GameResult,
    events,
    input::types::GameInput,
    render::state::RenderState,
    scenes::{
        types::SceneManager, InGameScene, MainMenuScene, OverworldScene, PalletTownOverworldScene,
    },
    settings::{AspectRatio, Settings},
    world,
};
use specs::WorldExt;
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
pub struct InputState {
    // TODO: Do we need both?
    // Could we do: gamepad_axis_x/y and set it to the configured stick?
    pub gamepad_axis_x: f32,
    pub gamepad_axis_y: f32,
}

pub struct GameState {
    pub world: specs::World,
    pub input_state: InputState,
    pub render_state: RenderState,
    pub settings: Settings,
}

impl GameState {
    fn new(ctx: &mut ggez::Context, settings: Settings) -> GameResult<Self> {
        Ok(Self {
            world: world::create_world(),
            input_state: InputState::default(),
            render_state: RenderState::new(ctx, &settings)?,
            settings,
        })
    }
}

pub struct GlobalState {
    pub scene_manager: SceneManager,
    pub game_state: GameState,
    pub delta_secs: f32,
}

impl GlobalState {
    pub fn new(ctx: &mut ggez::Context, settings: Settings) -> GameResult<Self> {
        let mut game_state = GameState::new(ctx, settings)?;

        let mut scene_manager = SceneManager::default();

        println!("update_stack: {}", scene_manager.update_stack().len());

        let scene = Rc::new(RefCell::new(InGameScene::new(&mut game_state, ctx)?));
        scene_manager.push(ctx, scene);

        println!("update_stack: {}", scene_manager.update_stack().len());

        let scene = Rc::new(RefCell::new(OverworldScene::new(&mut game_state, ctx)?));
        scene_manager.push(ctx, scene);

        println!("update_stack: {}", scene_manager.update_stack().len());

        let scene = Rc::new(RefCell::new(PalletTownOverworldScene::new(
            &mut game_state,
            ctx,
        )?));
        scene_manager.push(ctx, scene);

        println!("update_stack: {}", scene_manager.update_stack().len());

        Ok(Self {
            scene_manager,
            game_state,
            delta_secs: 0.,
        })
    }

    // TODO: Remove
    fn toggle_fullscreen(&mut self, ctx: &mut ggez::Context) -> GameResult {
        self.game_state.settings.video_settings.fullscreen_type =
            match self.game_state.settings.video_settings.fullscreen_type {
                ggez::conf::FullscreenType::Windowed => ggez::conf::FullscreenType::Desktop,
                ggez::conf::FullscreenType::Desktop => ggez::conf::FullscreenType::True,
                ggez::conf::FullscreenType::True => ggez::conf::FullscreenType::Windowed,
            };

        println!(
            "Set fullscreen: {:?}",
            self.game_state.settings.video_settings.fullscreen_type
        );
        self.game_state.settings.apply(ctx)?;

        Ok(())
    }
}

impl events::EventHandler for GlobalState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut scene_switch = None;

        for scene in self.scene_manager.update_stack() {
            scene_switch = scene
                .borrow_mut()
                .update(&mut self.game_state, ctx, self.delta_secs)?;
        }

        self.game_state.world.maintain();

        if let Some(scene_switch) = scene_switch {
            if let Some(scene) =
                self.scene_manager
                    .switch(&mut self.game_state, ctx, scene_switch)?
            {
                scene.borrow_mut().dispose(&mut self.game_state, ctx)?;
            }
        }

        Ok(())
    }

    fn draw(&self, ctx: &mut ggez::Context) -> GameResult {
        for scene in self.scene_manager.draw_stack() {
            scene.borrow().draw(&self.game_state, ctx)?;
        }

        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut ggez::Context, width: f32, height: f32) -> GameResult {
        if self.game_state.settings.video_settings.fullscreen_type
            == ggez::conf::FullscreenType::Windowed
        {
            self.game_state.settings.video_settings.windowed_width = width as usize;
            self.game_state.settings.video_settings.windowed_height = height as usize;
        }

        self.game_state.render_state.window_coords =
            ggez::graphics::Rect::new(0.0, 0.0, width, height);

        self.game_state
            .render_state
            .refresh(ctx, &self.game_state.settings.video_settings.aspect_ratio)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: ggez::input::keyboard::KeyCode,
    ) -> GameResult {
        if ggez::input::keyboard::is_mod_active(ctx, ggez::input::keyboard::KeyMods::CTRL) {
            match keycode {
                ggez::input::keyboard::KeyCode::Q => ggez::event::quit(ctx),
                ggez::input::keyboard::KeyCode::F => self.toggle_fullscreen(ctx)?,
                ggez::input::keyboard::KeyCode::A => {
                    let aspect_ratio = match self.game_state.settings.video_settings.aspect_ratio {
                        AspectRatio::Ratio16By9 => AspectRatio::Ratio4By3,
                        AspectRatio::Ratio4By3 => AspectRatio::PixelPerfect,
                        AspectRatio::PixelPerfect => AspectRatio::Stretch,
                        AspectRatio::Stretch => AspectRatio::Ratio16By9,
                    };
                    println!("Set aspect ratio: {:?}", aspect_ratio);

                    self.game_state.settings.video_settings.aspect_ratio = aspect_ratio;
                    self.game_state
                        .render_state
                        .refresh(ctx, &self.game_state.settings.video_settings.aspect_ratio)?;
                }
                ggez::input::keyboard::KeyCode::M => {
                    println!("Replacing top");
                    self.scene_manager.replace_top(
                        &mut self.game_state,
                        ctx,
                        Box::new(|ctx| Ok(Rc::new(RefCell::new(MainMenuScene::new(ctx)?)))),
                    )?;
                }
                // ggez::event::KeyCode::S => self.settings.save(),
                _ => {}
            }
        } else if let Some(game_input) =
            GameInput::from_keycode(&keycode, true, &self.game_state.settings)
        {
            let mut scene_switch = None;

            for scene in self.scene_manager.update_stack() {
                scene_switch =
                    scene
                        .borrow_mut()
                        .input(&mut self.game_state, ctx, game_input.clone())?;
            }

            if let Some(scene_switch) = scene_switch {
                if let Some(scene) =
                    self.scene_manager
                        .switch(&mut self.game_state, ctx, scene_switch)?
                {
                    scene.borrow_mut().dispose(&mut self.game_state, ctx)?;
                }
            }
        }

        Ok(())
    }

    fn key_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: ggez::input::keyboard::KeyCode,
    ) -> GameResult {
        if let Some(game_input) =
            GameInput::from_keycode(&keycode, false, &self.game_state.settings)
        {
            let mut scene_switch = None;

            for scene in self.scene_manager.update_stack() {
                scene_switch =
                    scene
                        .borrow_mut()
                        .input(&mut self.game_state, ctx, game_input.clone())?;
            }

            if let Some(scene_switch) = scene_switch {
                if let Some(scene) =
                    self.scene_manager
                        .switch(&mut self.game_state, ctx, scene_switch)?
                {
                    scene.borrow_mut().dispose(&mut self.game_state, ctx)?;
                }
            }
        }

        Ok(())
    }

    fn gamepad_button_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        btn: gilrs::Button,
        _id: ggez::input::gamepad::GamepadId,
    ) -> GameResult {
        if let Some(game_input) =
            GameInput::from_gamepad_button(&btn, true, &self.game_state.settings)
        {
            let mut scene_switch = None;

            for scene in self.scene_manager.update_stack() {
                scene_switch =
                    scene
                        .borrow_mut()
                        .input(&mut self.game_state, ctx, game_input.clone())?;
            }

            if let Some(scene_switch) = scene_switch {
                if let Some(scene) =
                    self.scene_manager
                        .switch(&mut self.game_state, ctx, scene_switch)?
                {
                    scene.borrow_mut().dispose(&mut self.game_state, ctx)?;
                }
            }
        }

        Ok(())
    }

    fn gamepad_button_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        btn: gilrs::Button,
        _id: ggez::input::gamepad::GamepadId,
    ) -> GameResult {
        if let Some(game_input) =
            GameInput::from_gamepad_button(&btn, false, &self.game_state.settings)
        {
            let mut scene_switch = None;

            for scene in self.scene_manager.update_stack() {
                scene_switch =
                    scene
                        .borrow_mut()
                        .input(&mut self.game_state, ctx, game_input.clone())?;
            }

            if let Some(scene_switch) = scene_switch {
                if let Some(scene) =
                    self.scene_manager
                        .switch(&mut self.game_state, ctx, scene_switch)?
                {
                    scene.borrow_mut().dispose(&mut self.game_state, ctx)?;
                }
            }
        }

        Ok(())
    }

    fn gamepad_axis_event(
        &mut self,
        ctx: &mut ggez::Context,
        axis: gilrs::Axis,
        value: f32,
        _id: ggez::input::gamepad::GamepadId,
    ) -> GameResult {
        let controller_settings = &self.game_state.settings.game_settings.controller_settings;

        if axis
            == controller_settings
                .controller_axis_mappings
                .controller_x_axis
        {
            self.game_state.input_state.gamepad_axis_x =
                if controller_settings.controller_axis_mappings.invert_x {
                    -value
                } else {
                    value
                };
        } else if axis
            == controller_settings
                .controller_axis_mappings
                .controller_y_axis
        {
            self.game_state.input_state.gamepad_axis_y =
                if controller_settings.controller_axis_mappings.invert_y {
                    -value
                } else {
                    value
                };
        } else {
            return Ok(());
        }

        let gamepad_axis_x = self.game_state.input_state.gamepad_axis_x;
        let gamepad_axis_y = self.game_state.input_state.gamepad_axis_y;
        let controller_stick_deadzone = controller_settings.controller_stick_deadzone;

        let game_input =
            GameInput::from_gamepad_axes(gamepad_axis_x, gamepad_axis_y, controller_stick_deadzone);

        let mut scene_switch = None;

        for scene in self.scene_manager.update_stack() {
            scene_switch =
                scene
                    .borrow_mut()
                    .input(&mut self.game_state, ctx, game_input.clone())?;
        }

        if let Some(scene_switch) = scene_switch {
            if let Some(scene) =
                self.scene_manager
                    .switch(&mut self.game_state, ctx, scene_switch)?
            {
                scene.borrow_mut().dispose(&mut self.game_state, ctx)?;
            }
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: ggez::event::winit_event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        println!("Mouse Button Down: {:?} @ [{}, {}]", button, x, y);
        Ok(())
    }
}

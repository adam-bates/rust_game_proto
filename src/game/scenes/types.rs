use super::{
    error::types::GameResult, game_state::GameState, input::types::GameInput, settings::Settings,
};

pub enum SceneSwitch {
    Pop,
    Push(Box<dyn Scene>),
    ReplaceTop(Box<dyn Scene>),
    ReplaceAll(Box<dyn Scene>),
}

// Structs that impl Scene will hold state that is "local" to that scene
pub trait Scene {
    fn update(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>>;

    fn draw(
        &self,
        game_state: &GameState,
        ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>>;

    fn input(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        input: GameInput,
    ) -> GameResult<Option<SceneSwitch>>;

    fn should_draw_previous(&self) -> bool {
        false
    }
}

#[derive(Default)]
pub struct SceneManager {
    scene_stack: Vec<Box<dyn Scene>>,
}

impl SceneManager {
    pub fn push(&mut self, scene: Box<dyn Scene>) {
        self.scene_stack.push(scene);
    }

    pub fn pop(&mut self) -> Option<Box<dyn Scene>> {
        self.scene_stack.pop()
    }

    pub fn unchecked_pop(&mut self) -> Box<dyn Scene> {
        self.pop()
            .expect("Failed to pop scene from empty SceneManager::scene_stack")
    }

    pub fn current(&self) -> Option<&Box<dyn Scene>> {
        self.scene_stack.last()
    }

    pub fn current_mut(&mut self) -> Option<&mut Box<dyn Scene>> {
        self.scene_stack.last_mut()
    }

    pub fn unchecked_current(&self) -> &dyn Scene {
        &**self
            .current()
            .expect("Failed to get current scene from empty SceneManager::scene_stack")
    }

    pub fn switch(&mut self, switch: SceneSwitch) -> Option<Box<dyn Scene>> {
        match switch {
            SceneSwitch::Pop => {
                let old = self.pop();
                old
            }
            SceneSwitch::Push(new) => {
                self.push(new);
                None
            }
            SceneSwitch::ReplaceTop(new) => {
                let old = self.pop();
                self.push(new);
                old
            }
            SceneSwitch::ReplaceAll(new) => {
                let mut old = None;
                while self.current().is_some() {
                    old = self.pop();
                }
                self.push(new);
                old
            }
        }
    }

    pub fn unchecked_switch(&mut self, switch: SceneSwitch) -> Option<Box<dyn Scene>> {
        match switch {
            SceneSwitch::Pop => {
                let old = self.unchecked_pop();
                Some(old)
            }
            SceneSwitch::Push(new) => {
                self.push(new);
                None
            }
            SceneSwitch::ReplaceTop(new) => {
                let old = self.unchecked_pop();
                self.push(new);
                Some(old)
            }
            SceneSwitch::ReplaceAll(new) => {
                let mut old = self.unchecked_pop();
                while self.current().is_some() {
                    old = self.unchecked_pop();
                }
                self.push(new);
                Some(old)
            }
        }
    }
}

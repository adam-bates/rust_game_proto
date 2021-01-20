use super::{error::types::GameResult, game_state::GameState, input::types::GameInput};

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
    pub fn push(&mut self, ctx: &mut ggez::Context, scene: Box<dyn Scene>) {
        ggez::graphics::clear(ctx, ggez::graphics::BLACK);
        self.scene_stack.push(scene);
    }

    pub fn pop(&mut self, ctx: &mut ggez::Context) -> Option<Box<dyn Scene>> {
        ggez::graphics::clear(ctx, ggez::graphics::BLACK);
        self.scene_stack.pop()
    }

    pub fn unchecked_pop(&mut self, ctx: &mut ggez::Context) -> Box<dyn Scene> {
        self.pop(ctx)
            .expect("Failed to pop scene from empty SceneManager::scene_stack")
    }

    pub fn replace_top(
        &mut self,
        ctx: &mut ggez::Context,
        scene: Box<dyn Scene>,
    ) -> Option<Box<dyn Scene>> {
        let top = self.pop(ctx);
        self.push(ctx, scene);
        top
    }

    pub fn unchecked_replace_top(
        &mut self,
        ctx: &mut ggez::Context,
        scene: Box<dyn Scene>,
    ) -> Box<dyn Scene> {
        let top = self.unchecked_pop(ctx);
        self.push(ctx, scene);
        top
    }

    pub fn replace_all(
        &mut self,
        ctx: &mut ggez::Context,
        scene: Box<dyn Scene>,
    ) -> Option<Box<dyn Scene>> {
        let mut last = None;
        while self.current().is_some() {
            last = self.pop(ctx);
        }
        self.push(ctx, scene);
        last
    }

    pub fn unchecked_replace_all(
        &mut self,
        ctx: &mut ggez::Context,
        scene: Box<dyn Scene>,
    ) -> Box<dyn Scene> {
        let mut last = self.unchecked_pop(ctx);
        while self.current().is_some() {
            last = self.unchecked_pop(ctx);
        }
        self.push(ctx, scene);
        last
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

    pub fn switch(
        &mut self,
        ctx: &mut ggez::Context,
        switch: SceneSwitch,
    ) -> Option<Box<dyn Scene>> {
        match switch {
            SceneSwitch::Pop => self.pop(ctx),
            SceneSwitch::Push(new) => {
                self.push(ctx, new);
                None
            }
            SceneSwitch::ReplaceTop(new) => self.replace_top(ctx, new),
            SceneSwitch::ReplaceAll(new) => self.replace_all(ctx, new),
        }
    }

    pub fn unchecked_switch(
        &mut self,
        ctx: &mut ggez::Context,
        switch: SceneSwitch,
    ) -> Option<Box<dyn Scene>> {
        match switch {
            SceneSwitch::Pop => Some(self.unchecked_pop(ctx)),
            SceneSwitch::Push(new) => {
                self.push(ctx, new);
                None
            }
            SceneSwitch::ReplaceTop(new) => Some(self.unchecked_replace_top(ctx, new)),
            SceneSwitch::ReplaceAll(new) => Some(self.unchecked_replace_all(ctx, new)),
        }
    }
}

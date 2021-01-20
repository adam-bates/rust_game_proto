use super::{error::types::GameResult, game_state::GameState, input::types::GameInput};

pub type SceneBuilder = Box<dyn Fn(&mut ggez::Context) -> GameResult<Box<dyn Scene>>>;

pub enum SceneSwitch {
    Pop,
    Push(SceneBuilder),
    ReplaceTop(SceneBuilder),
    ReplaceAll(SceneBuilder),
}

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
        scene_builder: SceneBuilder,
    ) -> GameResult {
        {
            let _ = self.pop(ctx);
        }

        let scene = scene_builder(ctx)?;
        self.push(ctx, scene);

        Ok(())
    }

    pub fn unchecked_replace_top(
        &mut self,
        ctx: &mut ggez::Context,
        scene_builder: SceneBuilder,
    ) -> GameResult {
        {
            let _ = self.unchecked_pop(ctx);
        }

        let scene = scene_builder(ctx)?;
        self.push(ctx, scene);

        Ok(())
    }

    pub fn replace_all(
        &mut self,
        ctx: &mut ggez::Context,
        scene_builder: SceneBuilder,
    ) -> GameResult {
        while self.current().is_some() {
            let _ = self.pop(ctx);
        }

        let scene = scene_builder(ctx)?;
        self.push(ctx, scene);

        Ok(())
    }

    pub fn unchecked_replace_all(
        &mut self,
        ctx: &mut ggez::Context,
        scene_builder: SceneBuilder,
    ) -> GameResult {
        while self.current().is_some() {
            let _ = self.unchecked_pop(ctx);
        }

        let scene = scene_builder(ctx)?;
        self.push(ctx, scene);

        Ok(())
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

    pub fn unchecked_current_mut(&mut self) -> &mut dyn Scene {
        &mut **self
            .current_mut()
            .expect("Failed to get current scene from empty SceneManager::scene_stack")
    }

    pub fn switch(
        &mut self,
        ctx: &mut ggez::Context,
        switch: SceneSwitch,
    ) -> GameResult<Option<Box<dyn Scene>>> {
        match switch {
            SceneSwitch::Pop => return Ok(self.pop(ctx)),
            SceneSwitch::Push(builder) => {
                let new = builder(ctx)?;
                self.push(ctx, new)
            }
            SceneSwitch::ReplaceTop(builder) => self.replace_top(ctx, builder)?,
            SceneSwitch::ReplaceAll(builder) => self.replace_all(ctx, builder)?,
        };

        Ok(None)
    }

    pub fn unchecked_switch(
        &mut self,
        ctx: &mut ggez::Context,
        switch: SceneSwitch,
    ) -> GameResult<Option<Box<dyn Scene>>> {
        match switch {
            SceneSwitch::Pop => return Ok(Some(self.unchecked_pop(ctx))),
            SceneSwitch::Push(builder) => {
                let new = builder(ctx)?;
                self.push(ctx, new);
            }
            SceneSwitch::ReplaceTop(builder) => self.unchecked_replace_top(ctx, builder)?,
            SceneSwitch::ReplaceAll(builder) => self.unchecked_replace_all(ctx, builder)?,
        };

        Ok(None)
    }
}

use super::{error::types::GameResult, game_state::GameState, input::types::GameInput};
use std::{cell::RefCell, rc::Rc};

pub type SceneBuilder =
    Box<dyn Fn(&mut GameState, &mut ggez::Context) -> GameResult<Rc<RefCell<dyn Scene>>>>;

pub enum SceneSwitch {
    Pop,
    Push(SceneBuilder),
    ReplaceTop(SceneBuilder),
    ReplaceAll(SceneBuilder),
}

pub trait Scene {
    fn dispose(&mut self, game_state: &mut GameState, ctx: &mut ggez::Context) -> GameResult;

    fn on_create(
        &mut self,
        _game_state: &mut GameState,
        _ctx: &mut ggez::Context,
    ) -> GameResult<Option<SceneSwitch>> {
        Ok(None)
    }

    fn update(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        delta_secs: f32,
    ) -> GameResult<Option<SceneSwitch>>;

    fn draw(&self, game_state: &GameState, ctx: &mut ggez::Context) -> GameResult;

    fn input(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        input: GameInput,
    ) -> GameResult<Option<SceneSwitch>>;

    fn should_draw_previous(&self) -> bool {
        false
    }

    fn should_update_previous(&self) -> bool {
        false
    }

    fn name(&self) -> &str;
}

fn build_update_stack_from(source_stack: &[Rc<RefCell<dyn Scene>>]) -> Vec<Rc<RefCell<dyn Scene>>> {
    let mut update_stack = vec![];

    if let Some((head, rest_of_stack)) = source_stack.split_last() {
        if head.borrow().should_update_previous() {
            update_stack.append(&mut build_update_stack_from(rest_of_stack));
        }

        update_stack.push(Rc::clone(head));
    }

    update_stack
}

fn build_draw_stack_from(source_stack: &[Rc<RefCell<dyn Scene>>]) -> Vec<Rc<RefCell<dyn Scene>>> {
    let mut draw_stack = vec![];

    if let Some((head, rest_of_stack)) = source_stack.split_last() {
        if head.borrow().should_draw_previous() {
            draw_stack.append(&mut build_draw_stack_from(rest_of_stack));
        }

        draw_stack.push(Rc::clone(head));
    }

    draw_stack
}

#[derive(Default)]
pub struct SceneManager {
    scene_stack: Vec<Rc<RefCell<dyn Scene>>>,
    update_stack: Vec<Rc<RefCell<dyn Scene>>>,
    draw_stack: Vec<Rc<RefCell<dyn Scene>>>,
}

impl std::fmt::Debug for SceneManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.scene_stack.len() == 0 {
            return f
                .write_str("SceneManager {{ scene_stack: [], update_top_n: 0, draw_top_n: 0 }}");
        }

        let first_scene_name = self.scene_stack[0].borrow().name().to_string();

        if self.scene_stack.len() == 1 {
            return f.write_str(&format!(
                "SceneManager {{ scene_stack: [{}], update_top_n: 1, draw_top_n: 1 }}",
                first_scene_name,
            ));
        }

        let scene_stack_names = self
            .scene_stack
            .iter()
            .skip(1)
            .map(|s| s.borrow().name().to_string())
            .fold(first_scene_name, |acc, name| format!("{}, {}", acc, name));

        f.write_str(&format!(
            "SceneManager {{ scene_stack: [{}], update_top_n: {}, draw_top_n: {} }}",
            scene_stack_names,
            self.update_stack.len(),
            self.draw_stack.len(),
        ))
    }
}

impl SceneManager {
    pub fn update_stack(&mut self) -> &mut Vec<Rc<RefCell<dyn Scene>>> {
        &mut self.update_stack
    }

    pub fn draw_stack(&self) -> &Vec<Rc<RefCell<dyn Scene>>> {
        &self.draw_stack
    }

    pub fn push(&mut self, ctx: &mut ggez::Context, scene: Rc<RefCell<dyn Scene>>) {
        ggez::graphics::clear(ctx, ggez::graphics::BLACK);

        if !scene.borrow().should_update_previous() {
            self.update_stack.clear();
        }
        self.update_stack.push(Rc::clone(&scene));

        if !scene.borrow().should_draw_previous() {
            self.draw_stack.clear();
        }
        self.draw_stack.push(Rc::clone(&scene));

        self.scene_stack.push(scene);
    }

    fn build_update_stack(&mut self) -> Vec<Rc<RefCell<dyn Scene>>> {
        build_update_stack_from(self.scene_stack.as_slice())
    }

    fn build_draw_stack(&mut self) -> Vec<Rc<RefCell<dyn Scene>>> {
        build_draw_stack_from(self.scene_stack.as_slice())
    }

    pub fn pop(&mut self, ctx: &mut ggez::Context) -> Option<Rc<RefCell<dyn Scene>>> {
        ggez::graphics::clear(ctx, ggez::graphics::BLACK);

        let popped = self.scene_stack.pop();
        self.update_stack.pop();
        self.draw_stack.pop();

        if let Some(popped) = popped {
            if !popped.borrow().should_update_previous() {
                // Wasn't updating previous
                if let Some(last) = self.scene_stack.last() {
                    if last.borrow().should_update_previous() {
                        // Requires filled out update_stack
                        self.update_stack = self.build_update_stack();
                    } else {
                        self.update_stack.push(Rc::clone(last));
                    }
                }
            }

            if !popped.borrow().should_draw_previous() {
                // Wasn't drawing previous
                if let Some(last) = self.scene_stack.last() {
                    if last.borrow().should_draw_previous() {
                        // Requires filled out draw_stack
                        self.draw_stack = self.build_draw_stack();
                    } else {
                        self.draw_stack.push(Rc::clone(last));
                    }
                }
            }

            return Some(popped);
        }

        None
    }

    pub fn unchecked_pop(&mut self, ctx: &mut ggez::Context) -> Rc<RefCell<dyn Scene>> {
        self.pop(ctx)
            .expect("Failed to pop scene from empty SceneManager::scene_stack")
    }

    pub fn replace_top(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        scene_builder: SceneBuilder,
    ) -> GameResult {
        {
            if let Some(old) = self.pop(ctx) {
                old.borrow_mut().dispose(game_state, ctx)?;
            }
        }

        let scene = scene_builder(game_state, ctx)?;
        self.push(ctx, scene);

        let switch = self
            .unchecked_current()
            .borrow_mut()
            .on_create(game_state, ctx)?;

        if let Some(switch) = switch {
            self.switch(game_state, ctx, switch)?;
        }

        Ok(())
    }

    pub fn unchecked_replace_top(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        scene_builder: SceneBuilder,
    ) -> GameResult {
        {
            self.unchecked_pop(ctx)
                .borrow_mut()
                .dispose(game_state, ctx)?;
        }

        let scene = scene_builder(game_state, ctx)?;
        self.push(ctx, scene);

        let switch = self
            .unchecked_current()
            .borrow_mut()
            .on_create(game_state, ctx)?;

        if let Some(switch) = switch {
            self.switch(game_state, ctx, switch)?;
        }

        Ok(())
    }

    pub fn replace_all(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        scene_builder: SceneBuilder,
    ) -> GameResult {
        while self.current().is_some() {
            if let Some(old) = self.pop(ctx) {
                old.borrow_mut().dispose(game_state, ctx)?;
            }
        }

        let scene = scene_builder(game_state, ctx)?;
        self.push(ctx, scene);

        let switch = self
            .unchecked_current()
            .borrow_mut()
            .on_create(game_state, ctx)?;

        if let Some(switch) = switch {
            self.switch(game_state, ctx, switch)?;
        }

        Ok(())
    }

    pub fn unchecked_replace_all(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        scene_builder: SceneBuilder,
    ) -> GameResult {
        while self.current().is_some() {
            self.unchecked_pop(ctx)
                .borrow_mut()
                .dispose(game_state, ctx)?;
        }

        let scene = scene_builder(game_state, ctx)?;
        self.push(ctx, scene);

        let switch = self
            .unchecked_current()
            .borrow_mut()
            .on_create(game_state, ctx)?;

        if let Some(switch) = switch {
            self.switch(game_state, ctx, switch)?;
        }

        Ok(())
    }

    pub fn current(&self) -> Option<&Rc<RefCell<dyn Scene>>> {
        self.scene_stack.last()
    }

    pub fn current_mut(&mut self) -> Option<&mut Rc<RefCell<dyn Scene>>> {
        self.scene_stack.last_mut()
    }

    pub fn unchecked_current(&self) -> &RefCell<dyn Scene> {
        &**self
            .current()
            .expect("Failed to get current scene from empty SceneManager::scene_stack")
    }

    pub fn previous(&self) -> Option<&Rc<RefCell<dyn Scene>>> {
        if let Some((_, rest)) = self.scene_stack.split_last() {
            return rest.last();
        }

        None
    }

    pub fn previous_mut(&mut self) -> Option<&mut Rc<RefCell<dyn Scene>>> {
        if let Some((_, rest)) = self.scene_stack.split_last_mut() {
            return rest.last_mut();
        }

        None
    }

    pub fn unchecked_previous(&self) -> &RefCell<dyn Scene> {
        &**self
            .scene_stack
            .split_last()
            .expect("Failed to split last scene from SceneManager::scene_stack")
            .1
            .last()
            .expect("Failed to get previous scene from SceneManager::scene_stack")
    }

    pub fn switch(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        switch: SceneSwitch,
    ) -> GameResult<Option<Rc<RefCell<dyn Scene>>>> {
        match switch {
            SceneSwitch::Pop => return Ok(self.pop(ctx)),
            SceneSwitch::Push(builder) => {
                let new = builder(game_state, ctx)?;
                self.push(ctx, new);

                let switch = self
                    .unchecked_current()
                    .borrow_mut()
                    .on_create(game_state, ctx)?;

                if let Some(switch) = switch {
                    self.switch(game_state, ctx, switch)?;
                }
            }
            SceneSwitch::ReplaceTop(builder) => self.replace_top(game_state, ctx, builder)?,
            SceneSwitch::ReplaceAll(builder) => self.replace_all(game_state, ctx, builder)?,
        };

        Ok(None)
    }

    pub fn unchecked_switch(
        &mut self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        switch: SceneSwitch,
    ) -> GameResult<Option<Rc<RefCell<dyn Scene>>>> {
        match switch {
            SceneSwitch::Pop => return Ok(Some(self.unchecked_pop(ctx))),
            SceneSwitch::Push(builder) => {
                let new = builder(game_state, ctx)?;
                self.push(ctx, new);

                let switch = self
                    .unchecked_current()
                    .borrow_mut()
                    .on_create(game_state, ctx)?;

                if let Some(switch) = switch {
                    self.switch(game_state, ctx, switch)?;
                }
            }
            SceneSwitch::ReplaceTop(builder) => {
                self.unchecked_replace_top(game_state, ctx, builder)?
            }
            SceneSwitch::ReplaceAll(builder) => {
                self.unchecked_replace_all(game_state, ctx, builder)?
            }
        };

        Ok(None)
    }
}

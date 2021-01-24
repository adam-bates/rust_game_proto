use specs::{Component, VecStorage};
use specs_derive::Component;
use std::sync::Arc;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub drawable: Arc<dyn ggez::graphics::Drawable + Sync + Send>,
    pub draw_params: ggez::graphics::DrawParam,
}

use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub drawable: Box<dyn ggez::graphics::Drawable + Sync + Send>,
    pub draw_params: ggez::graphics::DrawParam,
}

use specs::{Component, VecStorage};
use specs_derive::Component;
use std::sync::Arc;

#[derive(Component, Clone)]
#[storage(VecStorage)]
pub struct Drawable {
    pub drawable: Arc<dyn ggez::graphics::Drawable + Sync + Send>,
    pub draw_params: ggez::graphics::DrawParam,
}

impl std::fmt::Debug for Drawable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Drawable {{ draw_params: {:?}, drawable: ... }}",
            self.draw_params
        ))
    }
}

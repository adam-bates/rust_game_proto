use super::{
    components::{CurrentPosition, Drawable},
    config,
    resources::Camera,
};
use specs::Join;

pub struct UpdateDrawParamSystem;

impl<'a> specs::System<'a> for UpdateDrawParamSystem {
    type SystemData = (
        specs::Read<'a, Camera>,
        specs::ReadStorage<'a, CurrentPosition>,
        specs::WriteStorage<'a, Drawable>,
    );

    fn run(&mut self, (camera_r, current_position_c, mut drawable_c): Self::SystemData) {
        for (current_position, drawable) in (&current_position_c, &mut drawable_c).join() {
            // Help linter
            #[cfg(debug_assertions)]
            let current_position = current_position as &CurrentPosition;
            #[cfg(debug_assertions)]
            let drawable = drawable as &mut Drawable;

            drawable.draw_params = drawable.draw_params.dest([
                current_position.x * config::TILE_PIXELS_SIZE_F32,
                current_position.y * config::TILE_PIXELS_SIZE_F32,
            ]);
        }

        for drawable in (&mut drawable_c).join() {
            // Help linter
            #[cfg(debug_assertions)]
            let drawable = drawable as &mut Drawable;

            if let ggez::graphics::Transform::Values { ref mut dest, .. } =
                drawable.draw_params.trans
            {
                dest.x -= camera_r.x * config::TILE_PIXELS_SIZE_F32;
                dest.y -= camera_r.y * config::TILE_PIXELS_SIZE_F32;
            }
        }
    }
}

extern crate specs;

use super::components::{Drawable, SpriteSheet};
use specs::Join;

#[derive(Debug)]
pub struct UpdateSpriteSheetDrawParamSystem;

impl<'a> specs::System<'a> for UpdateSpriteSheetDrawParamSystem {
    type SystemData = (
        specs::ReadStorage<'a, SpriteSheet>,
        specs::WriteStorage<'a, Drawable>,
    );

    #[tracing::instrument(
        skip(sprite_sheet_c, drawable_c),
        name = "UpdateSpriteSheetDrawParamSystem",
        target = "UpdateSpriteSheetDrawParamSystem"
    )]
    fn run(&mut self, (sprite_sheet_c, mut drawable_c): Self::SystemData) {
        for (sprite_sheet, drawable) in (&sprite_sheet_c, &mut drawable_c).join() {
            // Help linter
            #[cfg(debug_assertions)]
            let sprite_sheet = sprite_sheet as &SpriteSheet;
            #[cfg(debug_assertions)]
            let drawable = drawable as &mut Drawable;

            drawable.draw_params = drawable.draw_params.src(sprite_sheet.image_src);
        }
    }
}

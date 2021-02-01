use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct SpriteRow {
    pub frames: usize,
    pub idx: usize,
}

impl SpriteRow {
    pub fn new(frames: usize) -> Self {
        Self { frames, idx: 0 }
    }

    pub fn next_frame(&mut self) {
        self.idx = (self.idx + 1) % self.frames;
    }

    pub fn prev_frame(&mut self) {
        if self.idx == 0 {
            self.idx = self.frames - 1;
        } else {
            self.idx -= 1;
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct SpriteSheet {
    pub sprite_rows: Vec<SpriteRow>,
    pub idx: usize,
    pub image_src: ggez::graphics::Rect,
}

impl SpriteSheet {
    pub fn new(sprite_rows: Vec<SpriteRow>) -> Self {
        let idx = 0;
        let image_src = Self::image_src(&sprite_rows, idx);

        Self {
            sprite_rows,
            idx,
            image_src,
        }
    }

    pub fn row(&self) -> &SpriteRow {
        &self.sprite_rows[self.idx]
    }

    pub fn set_frame(&mut self, idx: usize) {
        let sprite_row = &mut self.sprite_rows[self.idx];

        if sprite_row.idx != idx {
            sprite_row.idx = idx;
            self.refresh();
        }
    }

    pub fn next_frame(&mut self) {
        self.sprite_rows[self.idx].next_frame();
        self.refresh();
    }

    pub fn prev_frame(&mut self) {
        self.sprite_rows[self.idx].prev_frame();
        self.refresh();
    }

    pub fn set_row(&mut self, idx: usize) {
        if self.idx != idx {
            self.idx = idx;
            self.refresh();
        }
    }

    pub fn next_row(&mut self) {
        self.idx = (self.idx + 1) % self.sprite_rows.len();
        self.refresh();
    }

    pub fn prev_row(&mut self) {
        if self.idx == 0 {
            self.idx = self.sprite_rows.len() - 1;
        } else {
            self.idx -= 1;
        }

        self.refresh();
    }

    pub fn refresh(&mut self) {
        self.image_src = Self::image_src(&self.sprite_rows, self.idx);
    }

    fn image_src(sprite_rows: &Vec<SpriteRow>, idx: usize) -> ggez::graphics::Rect {
        let sprite_row = &sprite_rows[idx];

        let width = sprite_row.frames as f32;
        let height = sprite_rows.len() as f32;

        let x = sprite_row.idx as f32;
        let y = idx as f32;

        let inverse_width = 1. / width;
        let inverse_height = 1. / height;

        ggez::graphics::Rect::new(
            x * inverse_width,
            y * inverse_height,
            inverse_width,
            inverse_height,
        )
    }
}

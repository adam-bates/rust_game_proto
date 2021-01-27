use super::{components::Drawable, maps::TileType};

#[derive(Default, Debug)]
pub struct Tile {
    pub tile_type: Option<TileType>,
    pub entity: Option<specs::Entity>,
    pub overlay: Option<specs::Entity>,
}

pub struct Frame {
    pub idx: usize,
    pub tile_ids: Vec<usize>,
}

pub struct TileMap {
    pub tiles: Vec<Vec<Tile>>,
    pub tile_indices: Vec<usize>,
    pub overlay_indices: Vec<Option<usize>>,
    pub animation: Vec<Frame>,
    pub sprite_sheet_width: usize,
    pub sprite_sheet_height: usize,
    pub to_draw: Vec<Drawable>,
    pub overlay: ggez::graphics::spritebatch::SpriteBatch,
    pub background: ggez::graphics::spritebatch::SpriteBatch,
    pub background_param: ggez::graphics::DrawParam,
}

impl TileMap {
    pub fn dimensions(&self) -> (usize, usize) {
        let height = self.tiles.len();
        let width = if height == 0 { 0 } else { self.tiles[0].len() };

        (width, height)
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y][x]
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.tiles[y][x]
    }
}

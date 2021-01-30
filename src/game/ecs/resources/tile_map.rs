use super::{components::Drawable, maps::TileType};

#[derive(Default, Debug)]
pub struct Tile {
    pub tile_type: Option<TileType>,
    pub entity: Option<specs::Entity>,
}

pub struct Frame {
    pub idx: usize,
    pub tile_ids: Vec<usize>,
}

pub struct TileMap {
    pub tiles: Vec<Vec<Tile>>,
    pub tile_indices: Vec<Option<usize>>,
    pub overlay_indices: Vec<Option<usize>>,
    pub background_animation: Vec<Frame>,
    pub overlay_animation: Vec<Frame>,
    pub to_draw: Vec<Drawable>,
    pub overlay: ggez::graphics::spritebatch::SpriteBatch,
    pub overlay_width: usize,
    pub overlay_height: usize,
    pub background: ggez::graphics::spritebatch::SpriteBatch,
    pub background_width: usize,
    pub background_height: usize,
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

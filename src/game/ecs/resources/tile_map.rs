#[derive(Debug)]
pub enum TileType {
    Wall,
    Water,
    // ...
}

#[derive(Debug, Default)]
pub struct Tile {
    pub tile_type: Option<TileType>,
    pub entity: Option<specs::Entity>,
    pub overlay: Option<specs::Entity>,
}

#[derive(Debug)]
pub struct TileMap {
    pub tiles: Vec<Vec<Tile>>,
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

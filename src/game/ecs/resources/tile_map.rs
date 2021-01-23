pub enum TileType {
    Wall,
    Water,
    // ...
}

pub struct Tile {
    x: usize,
    y: usize,
    pub tile_type: Option<TileType>,
    pub entity: Option<specs::Entity>,
}

impl Tile {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            tile_type: None,
            entity: None,
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

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

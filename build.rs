use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::PathBuf, result::Result};

fn main() -> Result<(), Box<dyn Error>> {
    let root_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);

    let build_assets_path = root_path.join(PathBuf::from("build_assets"));

    for file in fs::read_dir(build_assets_path)? {
        let file = file?;
        let filename = PathBuf::from(file.file_name().to_str().unwrap().to_string())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let tile_map_json_path =
            root_path.join(PathBuf::from(format!("build_assets/{}.json", filename)));

        let tile_map_json_str = fs::read_to_string(tile_map_json_path)?;
        let tile_map_json_data: serde_json::Value = serde_json::from_str(&tile_map_json_str)?;

        let width = tile_map_json_data["width"].as_u64().unwrap();
        let height = tile_map_json_data["height"].as_u64().unwrap();
        let layers = tile_map_json_data["layers"].as_array().unwrap();

        let background_layer = layers[0].as_object().unwrap();
        let entity_layer = layers[1].as_object().unwrap();
        let overlay_layer = layers[2].as_object().unwrap();

        let background_tile_ids: Vec<u64> = background_layer["data"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() - 1) // Fix the offset by 1
            .collect();

        let entities = entity_layer["objects"].as_array().unwrap();
        let (player_x, player_y) = entities
            .iter()
            .find(|v| {
                let opt_name = v["name"].as_str();
                opt_name.is_some() && opt_name.unwrap() == "Player"
            })
            .map(|v| (v["x"].as_u64().unwrap(), v["y"].as_u64().unwrap()))
            .unwrap();

        let overlay_tile_ids: Vec<Option<u64>> = overlay_layer["data"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                let id = v.as_u64().unwrap();
                if id > 0 {
                    Some(id - 1)
                } else {
                    None
                }
            })
            .collect();

        let tile_sets = tile_map_json_data["tilesets"].as_array().unwrap();
        let tile_set = tile_sets[0].as_object().unwrap();

        let sprite_sheet_filename = PathBuf::from(tile_set["image"].as_str().unwrap().to_string())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let tiles: Vec<Tile> = tile_set["tiles"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                let id = v["id"].as_u64().unwrap();
                let tile_type = TileType::from(v["type"].as_str().unwrap());
                let animation = v["animation"].as_array().map(|values| {
                    values
                        .iter()
                        .map(|v| {
                            let tile_id = v["tileid"].as_u64().unwrap();

                            TileAnimationFrame { tile_id }
                        })
                        .collect()
                });

                Tile {
                    id,
                    tile_type,
                    animation,
                }
            })
            .collect();

        let tile_map = TileMap {
            width,
            height,
            player_x,
            player_y,
            sprite_sheet_filename,
            background_tile_ids,
            overlay_tile_ids,
            tiles,
        };

        let tile_map_bin_path =
            root_path.join(PathBuf::from(format!("assets/maps/{}.bin", filename)));

        bincode::serialize_into(fs::File::create(tile_map_bin_path)?, &tile_map)?;
    }

    println!("cargo:rerun-if-changed=assets/spritesheets");
    println!("cargo:rerun-if-changed=build_assets");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct TileMap {
    width: u64,
    height: u64,
    player_x: u64,
    player_y: u64,
    sprite_sheet_filename: String,
    background_tile_ids: Vec<u64>,
    overlay_tile_ids: Vec<Option<u64>>,
    tiles: Vec<Tile>,
}

#[derive(Serialize, Deserialize, Debug)]
enum TileType {
    Wall,
    Water,
}

impl From<&str> for TileType {
    fn from(string: &str) -> Self {
        match string {
            "Wall" => Self::Wall,
            "Water" => Self::Water,
            _ => panic!(format!("Unknown tile type: {}", string)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Tile {
    id: u64,
    tile_type: TileType,
    animation: Option<Vec<TileAnimationFrame>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TileAnimationFrame {
    tile_id: u64,
}

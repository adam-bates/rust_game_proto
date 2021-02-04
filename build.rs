use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs, path::PathBuf, result::Result};

fn main() -> Result<(), Box<dyn Error>> {
    let root_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);

    let build_assets_exports_path = root_path.join(PathBuf::from("build_assets/tiled/exports"));
    let binary_assets_path = root_path.join(PathBuf::from("assets/bin/maps"));

    // Clear old files in case of files that won't be reserialized
    for file in fs::read_dir(&binary_assets_path)? {
        let file = file?;
        fs::remove_file(file.path())?;
    }

    serialize_files(&binary_assets_path, build_assets_exports_path)?;

    println!("cargo:rerun-if-changed=assets/spritesheets");
    println!("cargo:rerun-if-changed=build_assets");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");

    Ok(())
}

fn serialize_files(
    binary_assets_path: &PathBuf,
    exports_dir: PathBuf,
) -> Result<(), Box<dyn Error>> {
    println!("Trying to serialize dir: {:#?}", exports_dir);

    for file in fs::read_dir(exports_dir)? {
        let file = file?;
        let file_type = file.file_type()?;

        if file_type.is_dir() {
            serialize_files(binary_assets_path, file.path())?;
        } else if file_type.is_file() {
            if let Some(ext) = file.path().extension() {
                if ext.to_str().unwrap() == "json" {
                    serialize_file(binary_assets_path, file)?;
                }
            }
        }
    }

    Ok(())
}

fn serialize_file(binary_assets_path: &PathBuf, file: fs::DirEntry) -> Result<(), Box<dyn Error>> {
    let filename = PathBuf::from(file.file_name().to_str().unwrap().to_string())
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let tile_map_json_str = fs::read_to_string(file.path())?;
    let tile_map_json_data: serde_json::Value = serde_json::from_str(&tile_map_json_str)?;

    let width = tile_map_json_data["width"].as_u64().unwrap();
    let height = tile_map_json_data["height"].as_u64().unwrap();
    let layers = tile_map_json_data["layers"].as_array().unwrap();

    let background_layer = layers[0].as_object().unwrap();
    let overlay_layer = layers[1].as_object().unwrap();

    let background_tile_ids: Vec<Option<u64>> = background_layer["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            let id = v.as_u64().unwrap();
            if id > 0 {
                Some(id - 1) // Fix the offset by 1
            } else {
                None
            }
        })
        .collect();

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

    let unwrapped_background_tile_ids: Vec<u64> = background_tile_ids
        .iter()
        .filter(|id| id.is_some())
        .map(|id| id.unwrap())
        .collect();

    let unwrapped_overlay_tile_ids: Vec<u64> = overlay_tile_ids
        .iter()
        .filter(|id| id.is_some())
        .map(|id| id.unwrap())
        .collect();

    let tilesets: Vec<&serde_json::Map<String, serde_json::Value>> = tile_map_json_data["tilesets"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(|v| v.as_object().unwrap())
        .collect();

    let background_tile_sets: Vec<(&&serde_json::Map<String, serde_json::Value>, u64, u64, u64)> =
        tilesets
            .iter()
            .map(|tile_set| {
                let min_id = tile_set["firstgid"].as_u64().unwrap() - 1;
                let max_id = min_id + tile_set["tilecount"].as_u64().unwrap() - 1;

                (tile_set, min_id, max_id)
            })
            .filter(|(_, min_id, max_id)| {
                unwrapped_background_tile_ids
                    .iter()
                    .any(|tile_id| *min_id <= *tile_id && *tile_id <= *max_id)
            })
            .map(|(tile_set, min_id, max_id)| {
                let width = tile_set["columns"].as_u64().unwrap();

                (tile_set, min_id, max_id, width)
            })
            .collect();

    let background_max_width = background_tile_sets
        .iter()
        .map(|(_, _, _, width)| *width)
        .max()
        .unwrap_or(0);

    let background_id_offset = background_tile_sets
        .iter()
        .map(|(_, min_id, _, _)| *min_id)
        .min()
        .unwrap_or(0);

    let mut post_offset = background_id_offset;
    let background_id_map =
        background_tile_sets
            .iter()
            .fold(HashMap::new(), |mut map, (_, min_id, max_id, width)| {
                let min_id = *min_id;
                let max_id = *max_id;
                let width = *width;
                let height = (max_id - min_id + 1) / width;

                for id in min_id..(max_id + 1) {
                    let pos = (id - min_id) / width;
                    let pos_scaled = pos * (background_max_width - width);
                    let id_offset = pos_scaled + post_offset - min_id;
                    let new_id = id + id_offset;

                    map.insert(id, new_id);
                }

                post_offset += background_max_width * height;

                return map;
            });

    let background_tile_sets: Vec<TileSet> = background_tile_sets
        .iter()
        .map(|(tile_set, min_id, _, _)| {
            let tiles: Vec<(u64, &serde_json::Map<String, serde_json::Value>)> =
                if tile_set.contains_key("tiles") {
                    tile_set["tiles"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| {
                            let tile = v.as_object().unwrap();
                            (tile["id"].as_u64().unwrap(), tile)
                        })
                        .collect()
                } else {
                    vec![]
                };

            let sprite_sheet_filename = tile_set["image"]
                .as_str()
                .unwrap()
                .to_string()
                .trim_start_matches("../../../assets")
                .to_string();

            let id_offset = min_id - background_id_offset;

            let tiles = tiles
                .into_iter()
                .map(|(id, v)| {
                    let tile_type = TileType::from(v["type"].as_str().unwrap());

                    let animation = if v.contains_key("animation") {
                        v["animation"].as_array().map(|values| {
                            values
                                .iter()
                                .map(|v| {
                                    let tile_id = *background_id_map
                                        .get(&(id_offset + v["tileid"].as_u64().unwrap()))
                                        .unwrap();

                                    TileAnimationFrame { tile_id }
                                })
                                .collect()
                        })
                    } else {
                        None
                    };

                    Tile {
                        id: *background_id_map.get(&(id_offset + id)).unwrap(),
                        tile_type,
                        animation,
                    }
                })
                .collect();

            TileSet {
                sprite_sheet_filename,
                tiles,
            }
        })
        .collect();

    let overlay_tile_sets: Vec<(&&serde_json::Map<String, serde_json::Value>, u64, u64, u64)> =
        tilesets
            .iter()
            .map(|tile_set| {
                let min_id = tile_set["firstgid"].as_u64().unwrap() - 1;
                let max_id = min_id + tile_set["tilecount"].as_u64().unwrap() - 1;

                (tile_set, min_id, max_id)
            })
            .filter(|(_, min_id, max_id)| {
                unwrapped_overlay_tile_ids
                    .iter()
                    .any(|tile_id| *min_id <= *tile_id && *tile_id <= *max_id)
            })
            .map(|(tile_set, min_id, max_id)| {
                let width = tile_set["columns"].as_u64().unwrap();

                (tile_set, min_id, max_id, width)
            })
            .collect();

    let overlay_max_width = overlay_tile_sets
        .iter()
        .map(|(_, _, _, width)| *width)
        .max()
        .unwrap_or(0);

    let overlay_id_offset = overlay_tile_sets
        .iter()
        .map(|(_, min_id, _, _)| *min_id)
        .min()
        .unwrap_or(0);

    let mut post_offset = overlay_id_offset;
    let overlay_id_map =
        overlay_tile_sets
            .iter()
            .fold(HashMap::new(), |mut map, (_, min_id, max_id, width)| {
                let min_id = *min_id;
                let max_id = *max_id;
                let width = *width;
                let height = (max_id - min_id + 1) / width;

                for id in min_id..(max_id + 1) {
                    let pos = (id - min_id) / width;
                    let pos_scaled = pos * (overlay_max_width - width);
                    let id_offset = pos_scaled + post_offset - min_id;
                    let new_id = id + id_offset;

                    map.insert(id, new_id);
                }

                post_offset += overlay_max_width * height;

                return map;
            });

    let overlay_tile_sets: Vec<TileSet> = overlay_tile_sets
        .iter()
        .map(|(tile_set, min_id, _, _)| {
            let tiles: Vec<(u64, &serde_json::Map<String, serde_json::Value>)> =
                if tile_set.contains_key("tiles") {
                    tile_set["tiles"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| {
                            let tile = v.as_object().unwrap();
                            (tile["id"].as_u64().unwrap(), tile)
                        })
                        .collect()
                } else {
                    vec![]
                };

            let sprite_sheet_filename = tile_set["image"]
                .as_str()
                .unwrap()
                .to_string()
                .trim_start_matches("../../../assets")
                .to_string();

            let id_offset = min_id - overlay_id_offset;

            let tiles = tiles
                .into_iter()
                .map(|(id, v)| {
                    let tile_type = TileType::from(v["type"].as_str().unwrap());

                    let animation = if v.contains_key("animation") {
                        v["animation"].as_array().map(|values| {
                            values
                                .iter()
                                .map(|v| {
                                    let tile_id = *overlay_id_map
                                        .get(&(id_offset + v["tileid"].as_u64().unwrap()))
                                        .unwrap();

                                    TileAnimationFrame { tile_id }
                                })
                                .collect()
                        })
                    } else {
                        None
                    };

                    Tile {
                        id: *overlay_id_map.get(&(id_offset + id)).unwrap(),
                        tile_type,
                        animation,
                    }
                })
                .collect();

            TileSet {
                sprite_sheet_filename,
                tiles,
            }
        })
        .collect();

    let background_tile_ids = background_tile_ids
        .into_iter()
        .map(|tile_id| match tile_id {
            Some(tile_id) => Some(background_id_map.get(&tile_id).unwrap() - background_id_offset),
            None => None,
        })
        .collect();

    let overlay_tile_ids = overlay_tile_ids
        .into_iter()
        .map(|tile_id| match tile_id {
            Some(tile_id) => Some(overlay_id_map.get(&tile_id).unwrap() - overlay_id_offset),
            None => None,
        })
        .collect();

    let background = TileLayer {
        tile_ids: background_tile_ids,
        tile_sets: background_tile_sets,
    };

    let overlay = TileLayer {
        tile_ids: overlay_tile_ids,
        tile_sets: overlay_tile_sets,
    };

    let tile_map = TileMap {
        width,
        height,
        background,
        overlay,
    };

    let tile_map_bin_path = binary_assets_path.join(PathBuf::from(format!("{}", filename)));

    bincode::serialize_into(fs::File::create(tile_map_bin_path)?, &tile_map)?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct TileMap {
    width: u64,
    height: u64,
    background: TileLayer,
    overlay: TileLayer,
}

#[derive(Serialize, Deserialize, Debug)]
struct TileLayer {
    tile_ids: Vec<Option<u64>>,
    tile_sets: Vec<TileSet>,
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
struct TileSet {
    sprite_sheet_filename: String,
    tiles: Vec<Tile>,
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

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
enum EntityType {
    Sign { id: u8 },
    Player,
    WiseOldMan,
}

impl From<&str> for EntityType {
    fn from(string: &str) -> Self {
        match string {
            "Sign1" => Self::Sign { id: 1 },
            "Sign2" => Self::Sign { id: 2 },
            "Sign3" => Self::Sign { id: 3 },
            "Sign4" => Self::Sign { id: 4 },
            "Sign5" => Self::Sign { id: 5 },
            "Player" => Self::Player,
            "WiseOldMan" => Self::WiseOldMan,
            _ => panic!(format!("Unknown entity type: {}", string)),
        }
    }
}

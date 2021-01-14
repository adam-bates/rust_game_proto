use super::config;
use super::error::types::{GameResult, Result};
use ggez::{
    filesystem::{File, Filesystem},
    vfs::VFS,
};
use std::path::Path;

mod factory;

pub fn new_filesystem() -> Result<Filesystem> {
    factory::new_filesystem()
}

pub fn file_exists(vfs: &Box<dyn VFS>, path: &Path) -> bool {
    vfs.metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

pub fn create_file(vfs: &Box<dyn VFS>, path: &Path) -> GameResult<File> {
    vfs.create(path).map(File::VfsFile)
}

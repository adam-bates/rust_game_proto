use super::config;
use super::error;
use ggez;

mod factory;

pub fn new_filesystem() -> error::types::Result<ggez::filesystem::Filesystem> {
    factory::new_filesystem()
}

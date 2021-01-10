extern crate chrono;

use super::error;

pub mod time {
    use chrono::Local;

    pub fn now_timestamp() -> i64 {
        Local::now().timestamp()
    }

    pub fn now_iso8601() -> String {
        Local::now().to_rfc3339()
    }
}

pub mod io {
    use super::error::types;
    use std::path::PathBuf;

    pub fn create_dir_if_not_exists(dir: PathBuf) -> types::IOResult {
        if let Err(_) = std::fs::read_dir(dir.clone()) {
            return std::fs::create_dir_all(dir);
        }

        Ok(())
    }
}

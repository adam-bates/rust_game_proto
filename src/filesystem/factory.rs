use super::{config, Filesystem, Result};
use directories::ProjectDirs;

const ASSETS_PATH: &str = "assets"; // TODO: config?

fn get_project_dirs() -> Result<ProjectDirs> {
    match ProjectDirs::from("", config::APPLICATION_AUTHOR, config::APPLICATION_ID) {
        Some(dirs) => Ok(dirs),
        _ => Err(Box::new(ggez::GameError::FilesystemError(String::from(
            "No valid home directory path could be retrieved.",
        )))),
    }
}

fn push_physical_fs<'a>(
    overlay_fs: &'a mut ggez::vfs::OverlayFS,
    path: std::path::PathBuf,
    read_only: bool,
) -> std::path::PathBuf {
    let physical_fs = ggez::vfs::PhysicalFS::new(&path, read_only);
    overlay_fs.push_back(Box::new(physical_fs));
    path
}

pub fn new_filesystem() -> Result<Filesystem> {
    let mut root_path = std::env::current_exe()?;
    let project_dirs = get_project_dirs()?;

    // Ditch the filename (if any)
    if root_path.file_name().is_some() {
        root_path.pop();
    }

    let mut overlay_fs = ggez::vfs::OverlayFS::new();

    // <game exe root>/assets/
    // or <cargo root>/assets/ if run with cargo
    let assets_path = {
        let mut path = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            std::path::PathBuf::from(manifest_dir)
        } else {
            root_path.clone()
        };
        path.push(ASSETS_PATH);

        push_physical_fs(&mut overlay_fs, path, true)
    };

    // Per-user data dir (roaming)
    // ~/.local/share/<app-id>/
    let user_data_path = push_physical_fs(
        &mut overlay_fs,
        project_dirs.data_dir().to_path_buf(),
        false,
    );

    // Local config dir
    // ~/.config/<app-id>/
    let user_config_path = push_physical_fs(
        &mut overlay_fs,
        project_dirs.config_dir().to_path_buf(),
        false,
    );

    Ok(ggez::filesystem::Filesystem {
        vfs: overlay_fs,
        assets_path,
        user_config_path,
        user_data_path,
    })
}

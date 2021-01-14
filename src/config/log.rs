use super::error::types;
use super::utils;

const MODULE_NAME: &str = super::APPLICATION_ID;
const MODULE_NAME_GFX: &str = "gfx";

#[cfg(not(debug_assertions))]
fn log_filename() -> String {
    format!(
        "{name}.{ext}",
        ext = super::LOG_FILE_EXT,
        name = utils::time::now_timestamp(),
    )
}

#[cfg(debug_assertions)]
pub struct LogOptions;

#[cfg(not(debug_assertions))]
pub struct LogOptions {
    filepath: std::path::PathBuf,
}

pub fn setup(_fs: &mut ggez::filesystem::Filesystem) -> types::Result<LogOptions> {
    #[allow(unused_mut)] // mut needed in release
    let mut dispatch = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{:<5}][{}] {}",
                utils::time::now_iso8601(),
                record.level(),
                record.target(),
                message
            ))
        })
        .level_for(MODULE_NAME, log::LevelFilter::Info)
        .level_for(MODULE_NAME_GFX, log::LevelFilter::Off);

    // Output to console in debug mode
    #[cfg(debug_assertions)]
    {
        dispatch.chain(std::io::stdout()).apply()?;
        return Ok(LogOptions);
    }

    // Output to ~/.config/<APPLICATION_ID>/.logs/<TIMESTAMP>.log file in release mode
    #[cfg(not(debug_assertions))]
    {
        let user_data_logs_path = _fs.user_data_path.join(super::LOGS_PATH_DIRNAME);

        {
            let user_data_vfs = _fs
                .find_vfs(_fs.user_data_path.as_path())
                .ok_or_else(|| "Unable to find user data directory")?;
            let dir_to_create = format!("/{}", super::LOGS_PATH_DIRNAME);
            let dir_path_to_create = std::path::Path::new(&dir_to_create);
            if let Err(e) = user_data_vfs.mkdir(dir_path_to_create) {
                println!(
                    "ERROR: Unable to create log directory [{}]: {}",
                    user_data_logs_path.to_string_lossy(),
                    e
                );
            }
        }

        let filename = log_filename();
        let filepath = user_data_logs_path.join(&filename);

        match fern::log_file(filepath.as_path()) {
            Ok(log_file) => dispatch = dispatch.chain(log_file),
            Err(e) => println!(
                "ERROR: Unable to create log file [{}]: {}",
                filepath.to_string_lossy(),
                e
            ),
        }

        dispatch.apply()?;

        return Ok(LogOptions { filepath });
    }
}

pub fn clean_up(_opts: LogOptions) -> types::Result {
    // Delete log file if nothing was written to it
    #[cfg(not(debug_assertions))]
    {
        if let Ok(bytes) = std::fs::read(_opts.filepath.as_path()) {
            if bytes.len() == 0 {
                std::fs::remove_file(_opts.filepath.as_path())?;
            }
        }
    }

    Ok(())
}

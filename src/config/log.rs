use super::error::types;
use super::utils;

#[cfg(not(debug_assertions))]
const LOG_FILE_PATH: &str = ".logs";

#[cfg(not(debug_assertions))]
const LOG_FILE_EXT: &str = "log";

const MODULE_NAME: &str = super::APPLICATION_ID;
const MODULE_NAME_GFX: &str = "gfx";

#[cfg(not(debug_assertions))]
fn log_filename() -> String {
    format!(
        "{name}.{ext}",
        ext = LOG_FILE_EXT,
        name = utils::time::now_timestamp(),
    )
}

#[cfg(debug_assertions)]
pub struct Output;

#[cfg(not(debug_assertions))]
pub struct Output {
    filepath: std::path::PathBuf,
}

pub fn setup(_config_path: &std::path::Path) -> types::LogSetupResult<Output> {
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

    #[cfg(debug_assertions)]
    {
        dispatch.chain(std::io::stdout()).apply()?;

        return Ok(Output);
    }

    #[cfg(not(debug_assertions))]
    {
        let logs_dir = _config_path.join(LOG_FILE_PATH);
        let filename = log_filename();

        let filepath = logs_dir.join(filename);

        if let Err(e) = utils::io::create_dir_if_not_exists(logs_dir.clone()) {
            println!(
                "ERROR: Unable to create log directory [{}]: {}",
                logs_dir.to_string_lossy(),
                e
            );
        }

        match fern::log_file(filepath.clone()) {
            Ok(log_file) => dispatch = dispatch.chain(log_file),
            Err(e) => println!(
                "ERROR: Unable to create log file [{}]: {}",
                filepath.to_string_lossy(),
                e
            ),
        }

        dispatch.apply()?;

        return Ok(Output { filepath });
    }
}

pub fn clean_up(_output: Output) -> types::Result {
    // Delete log file if nothing was written to it
    #[cfg(not(debug_assertions))]
    if let Ok(data) = std::fs::read(_output.filepath.clone()) {
        if data.len() == 0 {
            std::fs::remove_file(_output.filepath)?;
        }
    }

    Ok(())
}

use chrono::Local;

type Result<T = ()> = std::result::Result<T, log::SetLoggerError>;

pub fn setup() -> Result {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{:<5}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level_for(std::env!("CARGO_PKG_NAME"), log::LevelFilter::Info)
        .level_for("gfx", log::LevelFilter::Off)
        .level_for("threething", log::LevelFilter::Trace)
        // Hooks up console output.
        // TODO: env var for outputting to a file
        .chain(std::io::stdout())
        .apply()
}

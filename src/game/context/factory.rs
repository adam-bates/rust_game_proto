extern crate ggez;

use super::config;

// TODO: Build custom context
pub fn new_context() -> (ggez::Context, ggez::event::EventsLoop) {
    let cb = ggez::ContextBuilder::new(config::APPLICATION_ID, config::APPLICATION_AUTHOR)
        .add_resource_path("assets");

    cb.build().unwrap()

    // let mut fs = ggez::filesystem::Filesystem::new("", "")?;

    // for path in &self.paths {
    //     fs.mount(path, true);
    // }

    // for zipfile_bytes in self.memory_zip_files {
    //     fs.add_zip_file(std::io::Cursor::new(zipfile_bytes))?;
    // }

    // let config = if self.load_conf_file {
    //     fs.read_config().unwrap_or(self.conf)
    // } else {
    //     self.conf
    // };

    // ggez::Context::from_conf(config, fs)
}

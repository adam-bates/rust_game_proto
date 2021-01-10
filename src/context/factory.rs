extern crate ggez;

use super::config;

// TODO: Build custom context
pub fn new_context() -> (ggez::Context, ggez::event::EventsLoop) {
    let cb = ggez::ContextBuilder::new(config::APPLICATION_ID, config::APPLICATION_AUTHOR)
        .add_resource_path("assets");

    cb.build().unwrap()
}

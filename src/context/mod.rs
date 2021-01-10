use super::config;

mod factory;

pub fn new_context() -> (ggez::Context, ggez::event::EventsLoop) {
    factory::new_context()
}

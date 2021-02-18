use super::{components::Door, input::types::GameDirection};

#[derive(Default)]
pub struct DoorRequest {
    pub requesting: Option<Door>,
}

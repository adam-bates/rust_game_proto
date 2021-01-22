use super::input::types::GameDirection;

#[derive(Default)]
pub struct PlayerMovementRequest {
    pub last_requested_direction: Option<GameDirection>,
    pub last_requested_x_direction: Option<GameDirection>,
    pub last_requested_y_direction: Option<GameDirection>,
}

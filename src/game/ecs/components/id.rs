use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Id {
    value: String,
}

impl Id {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_lowercase(),
        }
    }
}

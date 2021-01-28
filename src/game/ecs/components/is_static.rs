use specs::{Component, NullStorage};
use specs_derive::Component;

#[derive(Default, Component, Debug)]
#[storage(NullStorage)]
pub struct IsStatic;

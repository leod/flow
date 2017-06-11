use component;

pub use types::ComponentId;
use types::{Coords, Orientation};

#[derive(Clone, Copy, Debug)]
pub enum SwitchType {
    On,
    Off
}

#[derive(Clone, Copy, Debug)]
pub enum Element {
    Switch(SwitchType),
    Capacitor,
    PowerSource
}

pub type Id = u32;

#[derive(Clone, Debug)]
pub struct Component {
    id: Id,
    top_left_position: Coords,
    orientation: Orientation,
    element: Element
}

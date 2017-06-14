use component;

pub use types::ComponentId;
use types::Dir;
use grid;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SwitchType {
    On,
    Off
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Element {
    Switch(SwitchType),
    Capacitor,
    PowerSource
}

pub type Id = u32;

#[derive(Clone, Debug)]
pub struct Component {
    id: Id,
    top_left_position: grid::Coords,
    direction: Dir,
    element: Element
}

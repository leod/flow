use std::collections::HashMap;

use grid::{self, Grid};
use component::{self, ComponentId, Element, Component};

pub struct Circuit {
    pub grid: Grid,
    pub components: HashMap<ComponentId, Component>
}

impl Circuit {
    pub fn new(grid: Grid, cs: HashMap<ComponentId, Component>) -> Circuit {
        Circuit {
            grid: grid,
            components: cs
        }
    }

    pub fn place_element(
        &mut self,
        p: grid::Coords,
        element: Element
    ) -> Option<ComponentId> {
        None          
    }
}

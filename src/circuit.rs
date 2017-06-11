use std::collections::HashMap;

use grid::{self, Grid};
use component::{self, ComponentId, Component};

pub struct Circuit {
    pub grid: Grid,
    pub components: HashMap<ComponentId, Component>
}

impl Circuit {
    pub fn new(grid: Grid, components: HashMap<ComponentId, Component>) -> Circuit {
        Circuit {
            grid: grid,
            components: components
        }
    }
}

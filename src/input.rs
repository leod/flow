pub use ggez::event::{MouseButton, MouseState, Keycode, Mod};

pub enum Input {
    MouseButtonDown {
        button: MouseButton,
        x: i32,
        y: i32
    },
    MouseButtonUp {
        button: MouseButton,
        x: i32,
        y: i32
    },
    MouseMotion {
        state: MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32
    },
    MouseWheel {
        x: i32,
        y: i32
    },
    KeyDown {
        keycode: Keycode,
        keymod: Mod,
        repeat: bool
    },
    KeyUp {
        keycode: Keycode,
        keymod: Mod,
        repeat: bool
    }
}


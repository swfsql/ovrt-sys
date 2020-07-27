//! Constants.

use super::wasm_bindgen;
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(i32)]
// TODO: check this rename from Devices
pub enum Device {
    /// None/World.
    World = 0,
    Hmd = 1,
    RightHand = 2,
    LeftHand = 3,
}

impl Default for Device {
    fn default() -> Self {
        Self::World
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(i32)]
// TODO: check this rename
pub enum WindowType {
    WebPage = 0,
    DesktopCapture = 1,
    WindowCapture = 2,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(i32)]
// TODO: check this rename from Settings
pub enum Setting {
    /// (Width in meters) (f64).
    Size = 0,
    /// (f64).
    Opacity = 1,
    /// (f64).
    Curvature = 2,
    /// (i32).
    Framerate = 3,
    /// (bool).
    EcoMode = 4,
    /// (bool).
    LookHiding = 5,
    /// (i32).
    AttachedDevice = 6,
}

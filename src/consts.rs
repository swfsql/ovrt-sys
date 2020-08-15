//! Constants.

use super::{types, wasm_bindgen};
#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
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

/// Represents kinds of window types.
///
/// See also: `types::WindowTypeValue`.
#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(i32)]
// TODO: check this rename
pub enum WindowType {
    WebPage = 0,
    DesktopCapture = 1,
    WindowCapture = 2,
}

impl WindowType {
    pub fn with(
        &self,
        value: types::window_type::Value,
    ) -> Option<types::window_type::WindowTypeValue> {
        use types::window_type::{Value::*, WindowTypeValue::*};
        match (self, value) {
            (Self::WebPage, WebContents(v)) => Some(WebPage(v)),
            (Self::DesktopCapture, I32(v)) => {
                Some(DesktopCapture(v))
            }
            (Self::WindowCapture, I32(v)) => {
                Some(WindowCapture(v))
            }
            _ => None,
        }
    }
}

impl From<&types::window_type::WindowTypeValue> for WindowType {
    fn from(
        valued: &types::window_type::WindowTypeValue,
    ) -> WindowType {
        use types::window_type::WindowTypeValue::*;
        match valued {
            WebPage(_) => WindowType::WebPage,
            DesktopCapture(_) => WindowType::DesktopCapture,
            WindowCapture(_) => WindowType::WindowCapture,
        }
    }
}

/// Represents kinds of setting values.
///
/// See also: `types::SettingValue`.
#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
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

impl Setting {
    pub fn with(
        &self,
        value: types::setting::Value,
    ) -> Option<types::setting::SettingValue> {
        use types::setting::{SettingValue::*, Value::*};
        match (self, value) {
            (Self::Size, F64(v)) => Some(Size(v)),
            (Self::Opacity, F64(v)) => Some(Opacity(v)),
            (Self::Curvature, F64(v)) => Some(Curvature(v)),
            (Self::Framerate, I32(v)) => Some(Framerate(v)),
            (Self::EcoMode, Bool(v)) => Some(EcoMode(v)),
            (Self::LookHiding, Bool(v)) => Some(LookHiding(v)),
            (Self::AttachedDevice, I32(v)) => {
                Some(AttachedDevice(v))
            }
            _ => None,
        }
    }
}

impl From<&types::setting::SettingValue> for Setting {
    fn from(valued: &types::setting::SettingValue) -> Setting {
        use types::setting::SettingValue::*;
        match valued {
            Size(_) => Setting::Size,
            Opacity(_) => Setting::Opacity,
            Curvature(_) => Setting::Curvature,
            Framerate(_) => Setting::Framerate,
            EcoMode(_) => Setting::EcoMode,
            LookHiding(_) => Setting::LookHiding,
            AttachedDevice(_) => Setting::AttachedDevice,
        }
    }
}

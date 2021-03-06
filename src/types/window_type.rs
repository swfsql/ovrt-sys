use crate::{
    api, consts,
    types::{self, Uid},
};

/// Any kind of value, to be coupled with `WindowType` into a `WindowTypeValue`.
#[derive(
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Value {
    WebContents(types::OVRWebContents),
    I32(i32),
}

impl From<&WindowTypeValue> for Value {
    fn from(valued: &WindowTypeValue) -> Value {
        use WindowTypeValue::*;
        match valued {
            WebPage(v) => Value::WebContents(v.clone()),
            DesktopCapture(v) => Value::I32(*v),
            WindowCapture(v) => Value::I32(*v),
        }
    }
}

/// This is a composition of `WindowType` and `Value`,
/// used to change contents of an overlay.
#[derive(
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum WindowTypeValue {
    WebPage(types::OVRWebContents),
    DesktopCapture(i32),
    WindowCapture(i32),
}

impl WindowTypeValue {
    /// Given a type kind and a value, tries to compose a `WindowTypeValue`.
    pub fn compose(
        kind: consts::WindowType,
        value: Value,
    ) -> Option<Self> {
        kind.with(value)
    }
    /// Extracts the type kind and value.
    pub fn decompose(&self) -> (consts::WindowType, Value) {
        (consts::WindowType::from(self), Value::from(self))
    }
    /// Uses this type kind and value to set an overlay content.
    ///
    /// TODO: if this presents a too high overhead, new abstractions
    /// can be introduced to call `set_contents_...` functions
    /// in a more direct manner.
    pub fn set_in_overlay(&self, uid: Uid) {
        use WindowTypeValue::*;
        unsafe {
            match self {
                WebPage(v) => api::set_contents_website(uid, v),
                DesktopCapture(v) => {
                    api::set_contents_desktop(uid, *v)
                }
                WindowCapture(v) => {
                    api::set_contents_window(uid, *v)
                }
            }
        }
    }
}

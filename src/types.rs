//! Types.

use super::wasm_bindgen;
use std::ops::{Deref, DerefMut};

/// OVROverlayTransform.
#[wasm_bindgen]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OVROverlayTransform {
    ///	X position of window.
    posX: f64,
    ///	Y position of window.
    posY: f64,
    ///	Z position of window.
    posZ: f64,
    ///	X rotation of window (EulerAngles).
    rotX: f64,
    ///	Y rotation of window (EulerAngles).
    rotY: f64,
    ///	Z rotation of window (EulerAngles).
    rotZ: f64,
    ///	Size of window (In meters).
    size: f64,
    ///	Opacity of window.
    opacity: f64,
    ///	Curvature of window.
    curvature: f64,
    ///	Framerate of window.
    framerate: i32,
    ///	Eco mode enabled or disabled.
    ecoMode: bool,
    ///	Look hiding enabled or disabled.
    lookHiding: bool,
    ///	Device window is attached to.
    attachedDevice: i32,
    ///	If overlay should save and automatically re-open next program start.
    shouldSave: bool,
}

/// OVRWebContents.
#[wasm_bindgen]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OVRWebContents {
    ///	URL of web page to display.
    url: String,
    ///	Width of overlay.
    width: i32,
    ///	Height of overlay.
    height: i32,
}

/// OVROverlayBounds.
///
/// Refer to Unity documentation for 'Bounds'.
#[wasm_bindgen]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OVROverlayBounds {
    ///	Vector3 Center - X.
    centerX: f64,
    ///	Vector3 Center - Y.
    centerY: f64,
    ///	Vector3 Center - Z.
    centerZ: f64,
    ///	Vector3 Extents - X.
    extentsX: f64,
    ///	Vector3 Extents - Y.
    extentsY: f64,
    ///	Vector3 Extents - Z.
    extentsZ: f64,
}

/// OVRFingerCurls.
///
/// 0 to 1.
#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OVRFingerCurls {
    ///	Thumb curl.
    thumb: f64,
    ///	Index curl.
    index: f64,
    ///	Middle curl.
    middle: f64,
    ///	ring curl.
    ring: f64,
    ///	ring pinky.
    pinky: f64,
}

/// OVRDeviceUpdate.
#[wasm_bindgen]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OVRDeviceUpdate {
    ///	If this is the active controller. (Always true for HMD).
    active: bool,
    ///	X position.
    posX: f64,
    ///	Y position.
    posY: f64,
    ///	Z position.
    posZ: f64,
    ///	X rotation (EulerAngles).
    rotX: f64,
    ///	Y rotation (EulerAngles).
    rotY: f64,
    ///	Z rotation (EulerAngles).
    rotZ: f64,
    ///	If trigger bind is pressed.
    triggerDown: bool,
    ///	If window move bind is pressed.
    windowMoveDown: bool,
    ///	If edit mode bind is pressed.
    editModeDown: bool,
    ///	Trackpad X tocuh position. (Left/Right)
    trackpadX: f64,
    ///	Trackpad Y touch position. (Up/Down)
    trackpadY: f64,
}

/// OVRTransformUpdate.
#[wasm_bindgen]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OVRTransformUpdate {
    ///	X position.
    posX: f64,
    ///	Y position.
    posY: f64,
    ///	Z position.
    posZ: f64,
    ///	X rotation (EulerAngles).
    rotX: f64,
    ///	Y rotation (EulerAngles).
    rotY: f64,
    ///	Z rotation (EulerAngles).
    rotZ: f64,
    ///	Overlay size.
    size: f64,
    ///	Overlay width in pixels.
    width: i32,
    ///	Overlay height in pixels.
    height: i32,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct KeyValuePairI32String {
    name: Option<i32>,
    value: Option<String>,
}
// pub type KeyValuePairI32String = KeyValuePair<i32, String>;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct KeyValuePair<K, V> {
    name: Option<K>,
    value: Option<V>,
}

// https://github.com/rustwasm/wasm-bindgen/issues/1775
// `String` fields cannot be `pub`, and require getter/setters.
impl<K, V> KeyValuePair<K, V>
where
    K: Deref + DerefMut,
    // <K as Deref>::Target: Sized,
    V: Deref + DerefMut,
    // <V as Deref>::Target: Sized,
{
    pub fn name(&self) -> Option<&K::Target> {
        self.name.as_deref()
    }
    pub fn name_mut(&mut self) -> Option<&mut K::Target> {
        self.name.as_deref_mut()
    }
    pub fn value(&self) -> Option<&V::Target> {
        self.value.as_deref()
    }
    pub fn value_mut(&mut self) -> Option<&mut V::Target> {
        self.value.as_deref_mut()
    }
}

//! Types.

use super::wasm_bindgen;

/// OVROverlayTransform.
#[wasm_bindgen]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OVROverlayTransform {
    ///	X position of window.
    pub posX: f64,
    ///	Y position of window.
    pub posY: f64,
    ///	Z position of window.
    pub posZ: f64,
    ///	X rotation of window (EulerAngles).
    pub rotX: f64,
    ///	Y rotation of window (EulerAngles).
    pub rotY: f64,
    ///	Z rotation of window (EulerAngles).
    pub rotZ: f64,
    ///	Size of window (In meters).
    pub size: f64,
    ///	Opacity of window.
    pub opacity: f64,
    ///	Curvature of window.
    pub curvature: f64,
    ///	Framerate of window.
    pub framerate: i32,
    ///	Eco mode enabled or disabled.
    pub ecoMode: bool,
    ///	Look hiding enabled or disabled.
    pub lookHiding: bool,
    ///	Device window is attached to.
    pub attachedDevice: i32,
    ///	If overlay should save and automatically re-open next program start.
    pub shouldSave: bool,
}

/// OVRWebContents.
#[wasm_bindgen]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OVRWebContents {
    ///	URL of web page to display.
    url: String,
    ///	Width of overlay.
    pub width: i32,
    ///	Height of overlay.
    pub height: i32,
}

// https://github.com/rustwasm/wasm-bindgen/issues/1775
// `String` fields cannot be `pub`, and require getter/setters.
impl OVRWebContents {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn url_mut(&mut self) -> &mut str {
        &mut self.url
    }
}

/// OVROverlayBounds.
///
/// Refer to Unity documentation for 'Bounds'.
#[wasm_bindgen]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OVROverlayBounds {
    ///	Vector3 Center - X.
    pub centerX: f64,
    ///	Vector3 Center - Y.
    pub centerY: f64,
    ///	Vector3 Center - Z.
    pub centerZ: f64,
    ///	Vector3 Extents - X.
    pub extentsX: f64,
    ///	Vector3 Extents - Y.
    pub extentsY: f64,
    ///	Vector3 Extents - Z.
    pub extentsZ: f64,
}

/// OVRFingerCurls.
///
/// 0 to 1.
#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OVRFingerCurls {
    ///	Thumb curl.
    pub thumb: f64,
    ///	Index curl.
    pub index: f64,
    ///	Middle curl.
    pub middle: f64,
    ///	ring curl.
    pub ring: f64,
    ///	ring pinky.
    pub pinky: f64,
}

/// OVRDeviceUpdate.
#[wasm_bindgen]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OVRDeviceUpdate {
    ///	If this is the active controller. (Always true for HMD).
    pub active: bool,
    ///	X position.
    pub posX: f64,
    ///	Y position.
    pub posY: f64,
    ///	Z position.
    pub posZ: f64,
    ///	X rotation (EulerAngles).
    pub rotX: f64,
    ///	Y rotation (EulerAngles).
    pub rotY: f64,
    ///	Z rotation (EulerAngles).
    pub rotZ: f64,
    ///	If trigger bind is pressed.
    pub triggerDown: bool,
    ///	If window move bind is pressed.
    pub windowMoveDown: bool,
    ///	If edit mode bind is pressed.
    pub editModeDown: bool,
    ///	Trackpad X tocuh position. (Left/Right)
    pub trackpadX: f64,
    ///	Trackpad Y touch position. (Up/Down)
    pub trackpadY: f64,
}

/// OVRTransformUpdate.
#[wasm_bindgen]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OVRTransformUpdate {
    ///	X position.
    pub posX: f64,
    ///	Y position.
    pub posY: f64,
    ///	Z position.
    pub posZ: f64,
    ///	X rotation (EulerAngles).
    pub rotX: f64,
    ///	Y rotation (EulerAngles).
    pub rotY: f64,
    ///	Z rotation (EulerAngles).
    pub rotZ: f64,
    ///	Overlay size.
    pub size: f64,
    ///	Overlay width in pixels.
    pub width: i32,
    ///	Overlay height in pixels.
    pub height: i32,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct KeyValuePairI32String {
    pub name: Option<i32>,
    value: Option<String>,
}

// https://github.com/rustwasm/wasm-bindgen/issues/1775
// `String` fields cannot be `pub`, and require getter/setters.
impl KeyValuePairI32String {
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
    pub fn value_mut(&mut self) -> Option<&mut str> {
        self.value.as_deref_mut()
    }
}

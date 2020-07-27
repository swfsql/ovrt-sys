//! Types.

use super::wasm_bindgen;

/// OVROverlayTransform.
#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
// TODO: try renaming into OvrOverlayTransform
pub struct OVROverlayTransform {
    ///	X position of window.
    #[wasm_bindgen(js_name = posX)]
    pub pos_x: f64,
    ///	Y position of window.
    #[wasm_bindgen(js_name = posY)]
    pub pos_y: f64,
    ///	Z position of window.
    #[wasm_bindgen(js_name = posZ)]
    pub pos_z: f64,
    ///	X rotation of window (EulerAngles).
    #[wasm_bindgen(js_name = rotX)]
    pub rot_x: f64,
    ///	Y rotation of window (EulerAngles).
    #[wasm_bindgen(js_name = rotY)]
    pub rot_y: f64,
    ///	Z rotation of window (EulerAngles).
    #[wasm_bindgen(js_name = rotZ)]
    pub rot_z: f64,
    ///	Size of window (In meters).
    pub size: f64,
    ///	Opacity of window.
    pub opacity: f64,
    ///	Curvature of window.
    pub curvature: f64,
    ///	Framerate of window.
    pub framerate: i32,
    ///	Eco mode enabled or disabled.
    #[wasm_bindgen(js_name = ecoMode)]
    pub eco_mode: bool,
    ///	Look hiding enabled or disabled.
    #[wasm_bindgen(js_name = lookHiding)]
    pub look_hiding: bool,
    ///	Device window is attached to.
    #[wasm_bindgen(js_name = attachedDevice)]
    pub attached_device: i32,
    ///	If overlay should save and automatically re-open next program start.
    #[wasm_bindgen(js_name = shouldSave)]
    pub should_save: bool,
}

/// OVRWebContents.
#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
// TODO: try renaming into OvrWebContents
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
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
// TODO: try renaming into OvrOverlayBounds
pub struct OVROverlayBounds {
    ///	Vector3 Center - X.
    #[wasm_bindgen(js_name = centerX)]
    pub center_x: f64,
    ///	Vector3 Center - Y.
    #[wasm_bindgen(js_name = centerY)]
    pub center_y: f64,
    ///	Vector3 Center - Z.
    #[wasm_bindgen(js_name = centerZ)]
    pub center_z: f64,
    ///	Vector3 Extents - X.
    #[wasm_bindgen(js_name = extentsX)]
    pub extents_x: f64,
    ///	Vector3 Extents - Y.
    #[wasm_bindgen(js_name = extentsY)]
    pub extents_y: f64,
    ///	Vector3 Extents - Z.
    #[wasm_bindgen(js_name = extentsZ)]
    pub extents_z: f64,
}

/// OVRFingerCurls.
///
/// 0 to 1.
#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
// TODO: try renaming into OvrFingerCurls
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
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
// TODO: try renaming into OvrDeviceUpdate
pub struct OVRDeviceUpdate {
    ///	If this is the active controller. (Always true for HMD).
    pub active: bool,
    ///	X position.
    #[wasm_bindgen(js_name = posX)]
    pub pos_x: f64,
    ///	Y position.
    #[wasm_bindgen(js_name = posY)]
    pub pos_y: f64,
    ///	Z position.
    #[wasm_bindgen(js_name = posZ)]
    pub pos_z: f64,
    ///	X rotation (EulerAngles).
    #[wasm_bindgen(js_name = rotX)]
    pub rot_x: f64,
    ///	Y rotation (EulerAngles).
    #[wasm_bindgen(js_name = rotY)]
    pub rot_y: f64,
    ///	Z rotation (EulerAngles).
    #[wasm_bindgen(js_name = rotZ)]
    pub rot_z: f64,
    ///	If trigger bind is pressed.
    #[wasm_bindgen(js_name = triggerDown)]
    pub trigger_down: bool,
    ///	If window move bind is pressed.
    #[wasm_bindgen(js_name = windowMoveDown)]
    pub window_move_down: bool,
    ///	If edit mode bind is pressed.
    #[wasm_bindgen(js_name = editModeDown)]
    pub edit_mode_down: bool,
    ///	Trackpad X tocuh position. (Left/Right)
    #[wasm_bindgen(js_name = trackpadX)]
    pub trackpad_x: f64,
    ///	Trackpad Y touch position. (Up/Down)
    #[wasm_bindgen(js_name = trackpadY)]
    pub trackpad_y: f64,
}

/// OVRTransformUpdate.
#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
// TODO: try renaming into OvrTransformUpdate
pub struct OVRTransformUpdate {
    ///	X position.
    #[wasm_bindgen(js_name = posX)]
    pub pos_x: f64,
    ///	Y position.
    #[wasm_bindgen(js_name = posY)]
    pub pos_y: f64,
    ///	Z position.
    #[wasm_bindgen(js_name = posZ)]
    pub pos_z: f64,
    ///	X rotation (EulerAngles).
    #[wasm_bindgen(js_name = rotX)]
    pub rot_x: f64,
    ///	Y rotation (EulerAngles).
    #[wasm_bindgen(js_name = rotY)]
    pub rot_y: f64,
    ///	Z rotation (EulerAngles).
    #[wasm_bindgen(js_name = rotZ)]
    pub rot_z: f64,
    ///	Overlay size.
    pub size: f64,
    ///	Overlay width in pixels.
    pub width: i32,
    ///	Overlay height in pixels.
    pub height: i32,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
// TODO: check this rename from KeyValuePair
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

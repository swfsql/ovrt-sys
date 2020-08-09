//! Types.

pub mod setting;
pub mod window_type;

pub use setting::SettingValue;
pub use window_type::WindowTypeValue;

#[derive(Debug, Default, Clone, druid::Data)]
pub struct Uid(pub i32);

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct P3 {
    /// X value.
    pub x: f64,
    /// Y value.
    pub y: f64,
    /// Z value.
    pub z: f64,
}

/// Position.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", transparent)]
pub struct Pos(pub P3);

/// Rotation (EulerAngles).
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", transparent)]
pub struct Rot(pub P3);

serde_with::with_prefix!(prefix_pos "pos");
serde_with::with_prefix!(prefix_rot "rot");
serde_with::with_prefix!(prefix_center "center");
serde_with::with_prefix!(prefix_extents "extents");

/// OVROverlayTransform.
// #[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
// TODO: try renaming into OvrOverlayTransform
pub struct OVROverlayTransform {
    /// Position of window.
    #[serde(flatten, with = "prefix_pos")]
    pos: Pos,
    /// rotation of window (EulerAngles).
    #[serde(flatten, with = "prefix_rot")]
    rot: Rot,
    /// Size of window (In meters).
    pub size: f64,
    /// Opacity of window.
    pub opacity: f64,
    /// Curvature of window.
    pub curvature: f64,
    /// Framerate of window.
    pub framerate: i32,
    /// Eco mode enabled or disabled.
    pub eco_mode: bool,
    /// Look hiding enabled or disabled.
    pub look_hiding: bool,
    /// Device window is attached to.
    pub attached_device: i32,
    /// If overlay should save and automatically re-open next program start.
    pub should_save: bool,
}

impl Default for OVROverlayTransform {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            rot: Default::default(),
            size: 0.25,
            opacity: 1.,
            curvature: 0.,
            framerate: 60,
            eco_mode: true,
            look_hiding: false,
            attached_device: 0,
            should_save: false,
        }
    }
}

/// OVRWebContents.
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
// TODO: try renaming into OvrWebContents
pub struct OVRWebContents {
    /// URL of web page to display.
    url: String,
    /// Width of overlay.
    pub width: i32,
    /// Height of overlay.
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
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
// TODO: try renaming into OvrOverlayBounds
pub struct OVROverlayBounds {
    /// Center - position.
    #[serde(flatten, with = "prefix_center")]
    pub center: Pos,
    /// Extents - position.
    #[serde(flatten, with = "prefix_extents")]
    pub extents: Pos,
}

/// OVRFingerCurls.
///
/// 0 to 1.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
// TODO: try renaming into OvrFingerCurls
pub struct OVRFingerCurls {
    /// Thumb curl.
    pub thumb: f64,
    /// Index curl.
    pub index: f64,
    /// Middle curl.
    pub middle: f64,
    /// ring curl.
    pub ring: f64,
    /// ring pinky.
    pub pinky: f64,
}

/// OVRDeviceUpdate.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
// TODO: try renaming into OvrDeviceUpdate
pub struct OVRDeviceUpdate {
    /// If this is the active controller. (Always true for HMD).
    pub active: bool,
    #[serde(flatten, with = "prefix_pos")]
    pub pos: Pos,
    #[serde(flatten, with = "prefix_rot")]
    pub rot: Rot,
    /// If trigger bind is pressed.
    pub trigger_down: bool,
    /// If window move bind is pressed.
    pub window_move_down: bool,
    /// If edit mode bind is pressed.
    pub edit_mode_down: bool,
    /// Trackpad X tocuh position. (Left/Right)
    pub trackpad_x: f64,
    /// Trackpad Y touch position. (Up/Down)
    pub trackpad_y: f64,
}

/// OVRTransformUpdate.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
// TODO: try renaming into OvrTransformUpdate
pub struct OVRTransformUpdate {
    #[serde(flatten, with = "prefix_pos")]
    pos: Pos,
    #[serde(flatten, with = "prefix_rot")]
    rot: Rot,
    /// Overlay size.
    pub size: f64,
    /// Overlay width in pixels.
    pub width: i32,
    /// Overlay height in pixels.
    pub height: i32,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
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

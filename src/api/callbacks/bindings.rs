//! Raw bindings into the javascript API.
//!
//! For using those functions, prefer `api::callbacks` instead.

use crate::types;
use crate::wasm_bindgen;

/// Spawn a new overlay.
///
/// This is private/hidden for safety. See `spawn_overlay` for more info.
///
/// Returns an uid.
#[wasm_bindgen(js_name = SpawnOverlayOvrtSysCallback)]
pub fn spawn_overlay(uid: i32) {
    super::spawn_overlay(types::Uid(uid))
}

/// Returns a list of open windows and their handles.
/// (If user has this option enabled).
///
/// Returns `windowTitles`.
#[wasm_bindgen(js_name = GetWindowTitlesOvrtSysCallback)]
// TODO: check accordingly to reference.
// reference: window.GetWindowTitles("completeIntervalWinTitles");
// window.GetWindowTitles("ovrtWinTitles");
pub fn get_window_titles(window_titles: String) {
    super::get_window_titles(window_titles)
}

/// (Used for SetContents monitorId).
///
/// Returns `monitorCount` (how many monitors are connected).
#[wasm_bindgen(js_name = GetMonitorCountOvrtSysCallback)]
// TODO: check accordingly to reference.
// reference: GetMonitorCount("ovrtMonitorTotal");
// window.GetMonitorCount(callback, data);
pub fn get_monitor_count(monitor_count: i32) {
    use std::convert::TryInto;
    super::get_monitor_count(
        monitor_count
            .try_into()
            .expect("unexpected negative monitor_count"),
    )
}

/// Get `OVROverlayTransform` of a specified overlay.
///
/// Returns `transformInfo`.
#[wasm_bindgen(js_name = GetOverlayTransformOvrtSysCallback)]
// TODO: check accordingly to reference.
// reference: window.GetOverlayTransform(String(uid), "ovrtWinDetailed");
pub fn get_overlay_transform(transform_info: String) {
    let data = serde_json::from_str::<types::OVROverlayTransform>(&transform_info)
        .expect("failed to deserialize");
    super::get_overlay_transform(data)
}

/// Get type of overlay.
/// (Browser, window capture, desktop capture).
///
/// Returns `type`.
#[wasm_bindgen(js_name = GetOverlayTypeOvrtSysCallback)]
// TODO: check accordingly to reference.
// reference: window.GetOverlayType(uid, callback);
pub fn get_overlay_type(type_: i32) {
    super::get_overlay_type(type_)
}

/// Get bounds of overlay bounding box.
/// (Refer to Unity documentation 'Bounds' section).
///
/// Returns `bounds`.
#[wasm_bindgen(js_name = GetOverlayBoundsOvrtSysCallback)]
// TODO: check accordingly to reference.
// reference: window.GetOverlayBounds(uid, callback);
pub fn get_overlay_bounds(bounds: String) {
    super::get_overlay_bounds(bounds)
}

/// Get finger curl positions.
///
/// Returns `curls`.
/// (Returns 0 for all values if user is in Simulator Mode).
#[wasm_bindgen(js_name = GetFingerCurlsOvrtSysCallback)]
// TODO: check accordingly to reference.
// reference: window.GetFingerCurls("completeFingerCurls");
pub fn get_finger_curls(curls: String) {
    super::get_finger_curls(curls)
}

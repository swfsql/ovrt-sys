pub mod bindings;

use crate::cmd::{sink::submit_cmd, Callback};
use crate::{log, types, v};

pub fn submit(callback: crate::cmd::Callback) {
    submit_cmd(crate::cmd::Command::Callback(callback))
}

// TODO: wrappers that will deal with deserialized data,
// like how events/mod deals with events/bindings.

/// Spawn a new overlay.
///
/// Returns an uid.
pub fn spawn_overlay(uid: types::Uid) {
    log!("callback spawn_overlay.", v(&uid));
    submit(Callback::FinishSpawnOverlay(uid));
}

/// Returns a list of open windows and their handles.
/// (If user has this option enabled).
///
/// Returns `windowTitles`.
// TODO: check accordingly to reference.
// reference: window.GetWindowTitles("completeIntervalWinTitles");
// window.GetWindowTitles("ovrtWinTitles");
pub fn get_window_titles(window_titles: String) {
    log!(
        "callback get_window_titles.",
        "window_titles:",
        &window_titles
    );
    submit(Callback::FinishGetWindowTitles(window_titles));
}

/// (Used for SetContents monitorId).
///
/// Returns `monitorCount` (how many monitors are connected).
// TODO: check accordingly to reference.
// reference: GetMonitorCount("ovrtMonitorTotal");
// window.GetMonitorCount(callback, data);
pub fn get_monitor_count(monitor_count: usize) {
    log!(
        "callback get_monitor_count.",
        "monitor_count:",
        v(monitor_count)
    );
    submit(Callback::FinishGetMonitorCount(monitor_count));
}

/// Get `OVROverlayTransform` of a specified overlay.
///
/// Returns `transformInfo`.
// TODO: check accordingly to reference.
// reference: window.GetOverlayTransform(String(uid), "ovrtWinDetailed");
pub fn get_overlay_transform(
    transform_info: types::OVROverlayTransform,
) {
    log!("callback get_overlay_transform.", v(&transform_info));
    submit(Callback::FinishGetOverlayTransform(transform_info));
}

/// Get type of overlay.
/// (Browser, window capture, desktop capture).
///
/// Returns `type`.
// TODO: check accordingly to reference.
// reference: window.GetOverlayType(uid, callback);
pub fn get_overlay_type(type_: i32) {
    log!("callback get_overlay_type.", "type_:", type_);
    submit(Callback::FinishGetOverlayType(type_));
}

/// Get bounds of overlay bounding box.
/// (Refer to Unity documentation 'Bounds' section).
///
/// Returns `bounds`.
// TODO: check accordingly to reference.
// reference: window.GetOverlayBounds(uid, callback);
pub fn get_overlay_bounds(bounds: String) {
    log!("callback get_overlay_bounds.", "bounds:", &bounds);
    submit(Callback::FinishGetOverlayBounds(bounds));
}

/// Get finger curl positions.
///
/// Returns `curls`.
/// (Returns 0 for all values if user is in Simulator Mode).
// TODO: check accordingly to reference.
// reference: window.GetFingerCurls("completeFingerCurls");
pub fn get_finger_curls(curls: String) {
    log!("callback get_finger_curls.", "curls:", &curls);
    submit(Callback::FinishGetFingerCurls(curls));
}

pub mod bindings;

use crate::{log, types, v};

// TODO: wrappers that will deal with deserialized data,
// like how events/mod deals with events/bindings.

/// Spawn a new overlay.
///
/// This is private/hidden for safety. See `spawn_overlay` for more info.
///
/// Returns an uid.
pub fn spawn_overlay(uid: types::Uid) {
    log!("callback spawn_overlay.", v(&uid));
    crate::ui::global_event_sink(None)
        .clone()
        .submit_command(crate::ui::cmd::FINISH_SPAWN_OVERLAY_CALLBACK, uid, None)
        .expect("command failed to submit");
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
    crate::ui::global_event_sink(None)
        .clone()
        .submit_command(
            crate::ui::cmd::FINISH_GET_WINDOW_TITLES_CALLBACK,
            window_titles,
            None,
        )
        .expect("command failed to submit");
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
    crate::ui::global_event_sink(None)
        .clone()
        .submit_command(
            crate::ui::cmd::FINISH_GET_MONITOR_COUNT_CALLBACK,
            monitor_count,
            None,
        )
        .expect("command failed to submit");
}

/// Get `OVROverlayTransform` of a specified overlay.
///
/// Returns `transformInfo`.
// TODO: check accordingly to reference.
// reference: window.GetOverlayTransform(String(uid), "ovrtWinDetailed");
pub fn get_overlay_transform(transform_info: types::OVROverlayTransform) {
    log!("callback get_overlay_transform.", v(&transform_info));
    crate::ui::global_event_sink(None)
        .clone()
        .submit_command(
            crate::ui::cmd::FINISH_GET_OVERLAY_TRANSFORM_CALLBACK,
            transform_info,
            None,
        )
        .expect("command failed to submit");
}

/// Get type of overlay.
/// (Browser, window capture, desktop capture).
///
/// Returns `type`.
// TODO: check accordingly to reference.
// reference: window.GetOverlayType(uid, callback);
pub fn get_overlay_type(type_: i32) {
    log!("callback get_overlay_type.", "type_:", type_);
    crate::ui::global_event_sink(None)
        .clone()
        .submit_command(
            crate::ui::cmd::FINISH_GET_OVERLAY_TYPE_CALLBACK,
            type_,
            None,
        )
        .expect("command failed to submit");
}

/// Get bounds of overlay bounding box.
/// (Refer to Unity documentation 'Bounds' section).
///
/// Returns `bounds`.
// TODO: check accordingly to reference.
// reference: window.GetOverlayBounds(uid, callback);
pub fn get_overlay_bounds(bounds: String) {
    log!("callback get_overlay_bounds.", "bounds:", &bounds);
    crate::ui::global_event_sink(None)
        .clone()
        .submit_command(
            crate::ui::cmd::FINISH_GET_OVERLAY_BOUNDS_CALLBACK,
            bounds,
            None,
        )
        .expect("command failed to submit");
}

/// Get finger curl positions.
///
/// Returns `curls`.
/// (Returns 0 for all values if user is in Simulator Mode).
// TODO: check accordingly to reference.
// reference: window.GetFingerCurls("completeFingerCurls");
pub fn get_finger_curls(curls: String) {
    log!("callback get_finger_curls.", "curls:", &curls);
    crate::ui::global_event_sink(None)
        .clone()
        .submit_command(
            crate::ui::cmd::FINISH_GET_FINGER_CURLS_CALLBACK,
            curls,
            None,
        )
        .expect("command failed to submit");
}

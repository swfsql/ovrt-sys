//! Note: At this time any API call that returns a value needs a callback assigned to return the value to, the 'callback' value will just be the string name of a function.
//!
//! Any API call that returns a value also had an additional string optional parameter, this can be used to share data back to the callback function.

pub mod bindings;
pub mod callbacks;

use super::types::{self, Uid};
use crate::cmd::{sink::submit_cmd, Notification};
use crate::{log, v};
use bindings as b;

pub fn submit(notification: crate::cmd::Notification) {
    submit_cmd(crate::cmd::Command::Notification(notification))
}

/// Spawn a new overlay.
///
// TODO: check accordingly to reference.
// reference: window.SpawnOverlay(JSON.stringify(transform), "ovrtWinSpawned");
pub fn spawn_overlay(
    transform_info: &types::OVROverlayTransform,
) -> Uid {
    let transform_info = serde_json::to_string(transform_info)
        .expect("serialization failed");
    log!("transform_str:", &transform_info);
    submit(Notification::StartSpawnOverlay);
    Uid(unsafe { b::spawn_overlay(transform_info) })
}

/// Spawn a new overlay.
///
// TODO: check accordingly to reference.
// reference: window.SpawnOverlay(JSON.stringify(transform), "ovrtWinSpawned");
pub fn spawn_overlay_with_callback(
    transform_info: &types::OVROverlayTransform,
) -> Uid {
    let transform_info = serde_json::to_string(transform_info)
        .expect("serialization failed");
    log!("transform_str:", &transform_info);
    submit(Notification::StartSpawnOverlay);
    Uid(unsafe {
        b::spawn_overlay_with_callback(
            transform_info,
            "SpawnOverlayOvrtSysCallback".into(),
        )
    })
}

/// Set contents of an overlay.
// TODO: check accordingly to reference.
// reference: window.SetContents(String(uid), Number(winData.type), normalizedContents);
pub fn set_contents_website(
    uid: Uid,
    contents: &types::OVRWebContents,
) {
    let contents = serde_json::to_string(contents)
        .expect("serialization failed");
    log!("contents", v(&contents));
    //
    // TODO: check if a notification should be sent
    //
    unsafe {
        b::set_contents_website(
            uid.0.to_string(),
            crate::consts::WindowType::WebPage as i32,
            contents,
        )
    }
}

/// Set contents of an overlay.
// TODO: check accordingly to reference.
// reference: window.SetContents(String(uid), Number(winData.type), normalizedContents);
pub fn set_contents_desktop(uid: Uid, monitor_id: i32) {
    //
    // TODO: check if a notification should be sent
    //
    unsafe {
        b::set_contents_desktop(
            uid.0.to_string(),
            crate::consts::WindowType::DesktopCapture as i32,
            monitor_id,
        )
    }
}

/// Set contents of an overlay.
// TODO: check accordingly to reference.
// reference: window.SetContents(String(uid), Number(winData.type), normalizedContents);
pub fn set_contents_window(uid: Uid, window_handle: i32) {
    //
    // TODO: check if a notification should be sent
    //
    unsafe {
        b::set_contents_window(
            uid.0.to_string(),
            crate::consts::WindowType::WindowCapture as i32,
            window_handle,
        )
    }
}

/// Returns a list of open windows and their handles.
/// (If user has this option enabled).
///
/// Returns `windowTitles`.
// TODO: check accordingly to reference.
// reference: window.GetWindowTitles("completeIntervalWinTitles");
// window.GetWindowTitles("ovrtWinTitles");
pub fn get_window_titles() -> types::KeyValuePairI32String {
    submit(Notification::StartGetWindowTitles);
    let window_titles = unsafe {
        b::get_window_titles(
            "GetWindowTitlesOvrtSysCallback".into(),
        )
    };
    serde_json::from_str::<types::KeyValuePairI32String>(
        &window_titles,
    )
    .expect("deserialization failed")
}

/// (Used for SetContents monitorId).
///
/// Returns `monitorCount` (how many monitors are connected).
// TODO: check accordingly to reference.
// reference: GetMonitorCount("ovrtMonitorTotal");
// window.GetMonitorCount(callback, data);
pub fn get_monitor_count() -> i32 {
    submit(Notification::StartGetMonitorCount);
    unsafe {
        b::get_monitor_count(
            "GetMonitorCountOvrtSysCallback".into(),
        )
    }
}

/// Refresh a browser page.
// TODO: check accordingly to reference.
// reference: window.Refresh(uid);
pub fn refresh(uid: Uid) {
    //
    // TODO: check if a notification should be sent
    //
    unsafe { b::refresh(uid.0) }
}

/// Get `OVROverlayTransform` of a specified overlay.
///
/// Returns `transformInfo`.
// TODO: check accordingly to reference.
// reference: window.GetOverlayTransform(String(uid), "ovrtWinDetailed");
pub fn get_overlay_transform(
    uid: Uid,
) -> types::OVROverlayTransform {
    submit(Notification::StartGetOverlayTransform(uid.clone()));
    let transform_info =
        unsafe { b::get_overlay_transform(uid.0) };
    serde_json::from_str::<types::OVROverlayTransform>(
        &transform_info,
    )
    .expect("deserialization failed")
}

/// Get `OVROverlayTransform` of a specified overlay.
///
/// Returns `transformInfo`.
// TODO: check accordingly to reference.
// reference: window.GetOverlayTransform(String(uid), "ovrtWinDetailed");
pub fn get_overlay_transform_with_callback(
    uid: Uid,
) -> types::OVROverlayTransform {
    submit(Notification::StartGetOverlayTransform(uid.clone()));
    let transform_info = unsafe {
        b::get_overlay_transform_with_callback(
            uid.0,
            "GetOverlayTransformOvrtSysCallback".into(),
        )
    };
    serde_json::from_str::<types::OVROverlayTransform>(
        &transform_info,
    )
    .expect("deserialization failed")
}

/// Get type of overlay.
/// (Browser, window capture, desktop capture).
///
/// Returns `type`.
// TODO: check accordingly to reference.
// reference: window.GetOverlayType(uid, callback);
pub fn get_overlay_type(uid: Uid) -> i32 {
    submit(Notification::StartGetOverlayType(uid.clone()));
    unsafe {
        b::get_overlay_type(
            uid.0,
            "GetOverlayTypeOvrtSysCallback".into(),
        )
    }
}

/// Get bounds of overlay bounding box.
/// (Refer to Unity documentation 'Bounds' section).
///
/// Returns `bounds`.
// TODO: check accordingly to reference.
// reference: window.GetOverlayBounds(uid, callback);
pub fn get_overlay_bounds(uid: Uid) -> types::OVROverlayBounds {
    submit(Notification::StartGetOverlayBounds(uid.clone()));
    let bounds = unsafe {
        b::get_overlay_bounds(
            uid.0,
            "GetOverlayBoundsOvrtSysCallback".into(),
        )
    };
    serde_json::from_str::<types::OVROverlayBounds>(&bounds)
        .expect("deserialization failed")
}

/// Get finger curl positions.
///
/// Returns `curls`.
/// (Returns 0 for all values if user is in Simulator Mode).
// TODO: check accordingly to reference.
// reference: window.GetFingerCurls("completeFingerCurls");
pub fn get_finger_curls() -> types::OVRFingerCurls {
    submit(Notification::StartGetFingerCurls);
    let curls = unsafe {
        b::get_finger_curls(
            "GetFingerCurlsOvrtSysCallback".into(),
        )
    };
    serde_json::from_str::<types::OVRFingerCurls>(&curls)
        .expect("deserialization failed")
}

/// Set position of an overlay.
// TODO: check accordingly to reference.
// reference: window.SetOverlayPosition(uid, pos.x, pos.y, pos.z);
pub fn set_overlay_position(uid: Uid, pos: types::Pos) {
    //
    // TODO: check if a notification should be sent
    //
    let pos = pos.0;
    unsafe {
        b::set_overlay_position(uid.0, pos.x, pos.y, pos.z)
    }
}

/// Set rotation of an overlay.
/// (EulerAngles).
// TODO: check accordingly to reference.
// reference: window.SetOverlayRotation(uid, rot.x, rot.y, rot.z);
pub fn set_overlay_rotation(uid: Uid, rot: types::Rot) {
    //
    // TODO: check if a notification should be sent
    //
    let rot = rot.0;
    unsafe {
        b::set_overlay_rotation(uid.0, rot.x, rot.y, rot.z)
    }
}

/// Set overlay setting.
// TODO: check accordingly to reference.
// reference: window.SetOverlaySetting(uid, setting, value);
pub fn set_overlay_setting_i32(
    uid: Uid,
    setting: i32,
    new_value: i32,
) {
    //
    // TODO: check if a notification should be sent
    //
    unsafe {
        b::set_overlay_setting_i32(uid.0, setting, new_value)
    }
}

/// Set overlay setting.
pub fn set_overlay_setting_f64(
    uid: Uid,
    setting: i32,
    new_value: f64,
) {
    //
    // TODO: check if a notification should be sent
    //
    unsafe {
        b::set_overlay_setting_f64(uid.0, setting, new_value)
    }
}

/// Set overlay setting.
pub fn set_overlay_setting_bool(
    uid: Uid,
    setting: i32,
    new_value: bool,
) {
    //
    // TODO: check if a notification should be sent
    //
    unsafe {
        b::set_overlay_setting_bool(uid.0, setting, new_value)
    }
}

/// Close the specified overlay.
// TODO: check accordingly to reference.
// reference: window.CloseOverlay(uid);
pub fn close_overlay(uid: Uid) {
    submit(Notification::StartCloseOverlay(uid.clone()));

    // https://github.com/swfsql/ovrt-sys/issues/6
    // unsafe { b::close_overlay(uid.0) }

    // https://github.com/swfsql/ovrt-sys/issues/6
    unsafe { b::close_overlay_str(uid.0.to_string()) }
}

/// Send device position/rotation data to the calling overlay.
/// (Refer to 'Events' below).
// TODO: check accordingly to reference.
// reference: window.SendDeviceData(enable);
pub fn send_device_data(enabled: bool) {
    //
    // TODO: check if a notification should be sent
    //
    unsafe { b::send_device_data(enabled) }
}

/// Send overlay position/rotation data to the calling overlay.
/// (Refer to 'Events' below).
// TODO: check accordingly to reference.
// reference: window.SendOverlayPositions(enable);
pub fn send_overlay_positions(enabled: bool) {
    //
    // TODO: check if a notification should be sent
    //
    unsafe { b::send_overlay_positions(enabled) }
}

/// Send message all other open browser instances.
/// (Calls 'ReceiveMessage').
// TODO: check accordingly to reference.
// reference: window.BroadcastMessage(JSON.stringify({broadcast: true,event: event,data: data,}));
// window.BroadcastMessage(JSON.stringify({broadcast: false,event: event,data: data,senderId: senderId,targetId: targetId,}));
//
// check send_message below
pub fn broadcast_message(message: String) {
    //
    // TODO: check if a notification should be sent
    //
    unsafe { b::broadcast_message(message) }
}

/// Send message to specific browser instance.
/// (Calls 'ReceiveMessage').
// TODO: check accordingly to reference.
// reference: none
// TODO: actually, it appears that this was merged into broadcast_message
// but now it has its own api call
pub fn send_message(uid: Uid, message: String) {
    unsafe { b::send_message(uid.0, message) }
}

/// Set if this overlay should receive keyboard inputs.
/// (This will block the keyboard working on OVR Toolkit windows!).
// TODO: check accordingly to reference.
// reference: none
pub fn set_keyboard_focus(enabled: bool) {
    //
    // TODO: check if a notification should be sent
    //
    unsafe { b::set_keyboard_focus(enabled) }
}

/// Sets the title of the browser that is visible in the Window List of OVR Toolkit.
// TODO: check accordingly to reference.
// reference: none
pub fn set_browser_title(title: String) {
    //
    // TODO: check if a notification should be sent
    //
    unsafe { b::set_browser_title(title) }
}

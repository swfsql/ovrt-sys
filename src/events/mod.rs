//! Events are functions you define that will be called by OVR Toolkit. When enabled, device updates will be called every frame.
//!
//! Some events are disabled by default, refer to the API section above for how to enable specific events.

pub mod bindings;

use crate::types::{self, Uid};
use crate::{log, v};

/// Sends HMD and left/right controller position and rotation, also shows the active controller.
/// (Needs to be enabled per overlay, refer to API above).
// TODO: check accordingly to reference.
// reference: function DevicePositionUpdate(deviceInfo) {
fn device_position_update(device_info: types::OVRDeviceUpdate) {
    log!(
        "event device_position_update.",
        "device_info: ",
        v(device_info)
    );
}

/// Receives messages from other browser instances.
// TODO: check accordingly to reference.
// reference: function ReceiveMessage(message) {
fn receive_message(message: String) {
    log!("event receive_message", "message: ", message);
}

/// If the user is interacting with the current overlay.
/// (Mouse over).
// TODO: check accordingly to reference.
// reference: function InteractionStateChanged(isInteracting) {
fn interaction_state_changed(is_interacting: bool) {
    log!(
        "event interaction_state_changed.",
        "is_interacting: ",
        is_interacting
    );
}

/// Called when an overlay is spawned.
// TODO: check accordingly to reference.
// reference: function OverlayOpened(uid) {
fn overlay_opened(uid: Uid) {
    log!("event overlay_opened.", "Uid:", uid.0);
}

/// Called when an overlay is closed.
// TODO: check accordingly to reference.
// reference: function OverlayClosed(uid) {
fn overlay_closed(uid: Uid) {
    log!("event overlay_closed.", "Uid: ", uid.0);
}

/// Called when an overlay is moved or its size changes.
/// (Needs to be enabled per overlay, refer to API above).
// TODO: check accordingly to reference.
// reference: function OverlayTransformChanged(updateData) {
fn overlay_transform_changed(uid: Uid, data: types::OVRTransformUpdate) {
    log!(
        "event overlay_transform_changed.",
        "Uid: ",
        uid.0,
        "data: ",
        v(data)
    );
}

/// Called when the API has finished injecting into the browser and the API can now be used.
// TODO: check accordingly to reference.
// reference: none
fn api_init() {
    log!("event api_init");
}

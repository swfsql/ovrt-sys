//! Events are functions you define that will be called by OVR Toolkit. When enabled, device updates will be called every frame.
//!
//! Some events are disabled by default, refer to the API section above for how to enable specific events.

pub mod bindings;

use crate::cmd::{sink::submit_cmd, Event, EventResponse};
use crate::types::{self, Uid};

pub fn submit_event_response(
    evt_resp: crate::cmd::EventResponse,
) {
    submit_cmd(crate::cmd::Command::EventResponse(evt_resp))
}

pub fn submit_event(evt: crate::cmd::Event) {
    submit_cmd(crate::cmd::Command::Event(evt))
}

/// Sends HMD and left/right controller position and rotation, also shows the active controller.
/// (Needs to be enabled per overlay, refer to API above).
// TODO: check accordingly to reference.
// reference: function DevicePositionUpdate(deviceInfo) {
fn device_position_update(device_info: types::OVRDeviceUpdate) {
    submit_event(Event::DevicePositionUpdate(device_info))
}

/// Receives messages from other browser instances.
// TODO: check accordingly to reference.
// reference: function ReceiveMessage(message) {
fn receive_message(message: String) {
    submit_event(Event::ReceiveMessage(message))
}

/// If the user is interacting with the current overlay.
/// (Mouse over).
// TODO: check accordingly to reference.
// reference: function InteractionStateChanged(isInteracting) {
fn interaction_state_changed(is_interacting: bool) {
    submit_event(Event::InteractionStateChanged {
        is_interacting,
    })
}

/// Called when an overlay is spawned.
// TODO: check accordingly to reference.
// reference: function OverlayOpened(uid) {
fn overlay_opened(uid: Uid) {
    submit_event_response(EventResponse::FinishSpawnOverlay(
        uid,
    ));
}

/// Called when an overlay is closed.
// TODO: check accordingly to reference.
// reference: function OverlayClosed(uid) {
fn overlay_closed(uid: Uid) {
    submit_event_response(EventResponse::FinishCloseOverlay(
        uid,
    ));
}

/// Called when an overlay is moved or its size changes.
/// (Needs to be enabled per overlay, refer to API above).
// TODO: check accordingly to reference.
// reference: function OverlayTransformChanged(updateData) {
fn overlay_transform_changed(
    uid: Uid,
    data: types::OVRTransformUpdate,
) {
    submit_event(Event::OverlayTransformChanged(uid, data));
}

/// Called when the API has finished injecting into the browser and the API can now be used.
// TODO: check accordingly to reference.
// reference: none
fn api_init() {
    submit_event(Event::ApiInit);
}

//! Raw bindings into the javascript API.
//!
//! For using those functions, prefer `events` instead.

use crate::{
    types::{self, Uid},
    wasm_bindgen,
};

/// Sends HMD and left/right controller position and rotation, also shows the active controller.
/// (Needs to be enabled per overlay, refer to API above).
// https://github.com/swfsql/ovrt-sys/issues/2
#[wasm_bindgen(js_name = DevicePositionUpdate)]
// TODO: check accordingly to reference.
// reference: function DevicePositionUpdate(deviceInfo) {
pub fn device_position_update(device_info: String) {
    let device_info = serde_json::from_str::<
        types::OVRDeviceUpdate,
    >(&device_info)
    .expect("failed to deserialize");
    super::device_position_update(device_info)
}

/// Receives messages from other browser instances.
#[wasm_bindgen(js_name = ReceiveMessage)]
// TODO: check accordingly to reference.
// reference: function ReceiveMessage(message) {
pub fn receive_message(message: String) {
    super::receive_message(message)
}

/// If the user is interacting with the current overlay.
/// (Mouse over).
#[wasm_bindgen(js_name = InteractionStateChanged)]
// TODO: check accordingly to reference.
// reference: function InteractionStateChanged(isInteracting) {
pub fn interaction_state_changed(is_interacting: bool) {
    super::interaction_state_changed(is_interacting)
}

/// Called when an overlay is spawned.
#[wasm_bindgen(js_name = OverlayOpened)]
// TODO: check accordingly to reference.
// reference: function OverlayOpened(uid) {
pub fn overlay_opened(uid: i32) {
    super::overlay_opened(Uid(uid))
}

/// Called when an overlay is closed.
#[wasm_bindgen(js_name = OverlayClosed)]
// TODO: check accordingly to reference.
// reference: function OverlayClosed(uid) {
pub fn overlay_closed(uid: i32) {
    super::overlay_closed(Uid(uid))
}

/// Called when an overlay is moved or its size changes.
/// (Needs to be enabled per overlay, refer to API above).
// https://github.com/swfsql/ovrt-sys/issues/2
#[wasm_bindgen(js_name = OverlayTransformChanged)]
// TODO: check accordingly to reference.
// reference: function OverlayTransformChanged(updateData) {
pub fn overlay_transform_changed(uid: i32, data: String) {
    let data =
        serde_json::from_str::<types::OVRTransformUpdate>(&data)
            .expect("failed to deserialize");
    super::overlay_transform_changed(Uid(uid), data)
}

/// Called when the API has finished injecting into the browser and the API can now be used.
#[wasm_bindgen(js_name = APIInit)]
// TODO: check accordingly to reference.
// reference: none
pub fn api_init() {
    super::api_init()
}

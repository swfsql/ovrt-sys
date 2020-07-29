//! Events are functions you define that will be called by OVR Toolkit. When enabled, device updates will be called every frame.
//!
//! Some events are disabled by default, refer to the API section above for how to enable specific events.

use super::{types, wasm_bindgen};
use web_sys::console::{log_1, log_2};

/// Sends HMD and left/right controller position and rotation, also shows the active controller.
/// (Needs to be enabled per overlay, refer to API above).
#[wasm_bindgen(js_name = DevicePositionUpdate)]
// TODO: check accordingly to reference.
// reference: function DevicePositionUpdate(deviceInfo) {
pub fn device_position_update(_device_info: types::OVRDeviceUpdate) {
    unsafe { log_1(&"event device_position_update".into()) };
}

/// Receives messages from other browser instances.
#[wasm_bindgen(js_name = ReceiveMessage)]
// TODO: check accordingly to reference.
// reference: function ReceiveMessage(message) {
pub fn receive_message(message: String) {
    unsafe { log_2(&"event receive_message:".into(), &message.into()) };
}

/// If the user is interacting with the current overlay.
/// (Mouse over).
#[wasm_bindgen(js_name = InteractionStateChanged)]
// TODO: check accordingly to reference.
// reference: function InteractionStateChanged(isInteracting) {
pub fn interaction_state_changed(is_interacting: bool) {
    unsafe {
        log_2(
            &"event interaction_state_changed:".into(),
            &is_interacting.into(),
        )
    };
}

/// Called when an overlay is spawned.
#[wasm_bindgen(js_name = OverlayOpened)]
// TODO: check accordingly to reference.
// reference: function OverlayOpened(uid) {
pub fn overlay_opened(uid: i32) {
    unsafe { log_2(&"event overlay_opened:".into(), &uid.into()) };
}

/// Called when an overlay is closed.
#[wasm_bindgen(js_name = OverlayClosed)]
// TODO: check accordingly to reference.
// reference: function OverlayClosed(uid) {
pub fn overlay_closed(uid: i32) {
    unsafe { log_2(&"event overlay_closed:".into(), &uid.into()) };
}

/// Called when an overlay is moved or its size changes.
/// (Needs to be enabled per overlay, refer to API above).
#[wasm_bindgen(js_name = OverlayTransformChanged)]
// TODO: check accordingly to reference.
// reference: function OverlayTransformChanged(updateData) {
pub fn overlay_transform_changed(uid: i32, _data: types::OVRTransformUpdate) {
    unsafe { log_2(&"event overlay_transform_changed:".into(), &uid.into()) };
}

/// Called when the API has finished injecting into the browser and the API can now be used.
#[wasm_bindgen(js_name = APIInit)]
// TODO: check accordingly to reference.
// reference: none
pub fn api_init() {
    unsafe { log_1(&"event api_init".into()) };
}

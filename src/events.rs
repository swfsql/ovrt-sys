//! Events are functions you define that will be called by OVR Toolkit. When enabled, device updates will be called every frame.
//!
//! Some events are disabled by default, refer to the API section above for how to enable specific events.

use super::{types, wasm_bindgen};

#[wasm_bindgen]
extern "C" {

    /// Sends HMD and left/right controller position and rotation, also shows the active controller.
    /// (Needs to be enabled per overlay, refer to API above).
    #[wasm_bindgen(js_name = DevicePositionUpdate)]
    // TODO: check accordingly to reference.
    // reference: function DevicePositionUpdate(deviceInfo) {
    pub fn device_position_update(deviceInfo: types::OVRDeviceUpdate);

    /// Receives messages from other browser instances.
    #[wasm_bindgen(js_name = ReceiveMessage)]
    // TODO: check accordingly to reference.
    // reference: function ReceiveMessage(message) {
    pub fn receive_message(message: String);

    /// If the user is interacting with the current overlay.
    /// (Mouse over).
    #[wasm_bindgen(js_name = InteractionStateChanged)]
    // TODO: check accordingly to reference.
    // reference: function InteractionStateChanged(isInteracting) {
    pub fn interaction_state_changed(isInteracting: bool);

    /// Called when an overlay is spawned.
    #[wasm_bindgen(js_name = OverlayOpened)]
    // TODO: check accordingly to reference.
    // reference: function OverlayOpened(uid) {
    pub fn overlay_opened(uid: i32);

    /// Called when an overlay is closed.
    #[wasm_bindgen(js_name = OverlayClosed)]
    // TODO: check accordingly to reference.
    // reference: function OverlayClosed(uid) {
    pub fn overlay_closed(uid: i32);

    /// Called when an overlay is moved or its size changes.
    /// (Needs to be enabled per overlay, refer to API above).
    #[wasm_bindgen(js_name = OverlayTransformChanged)]
    // TODO: check accordingly to reference.
    // reference: function OverlayTransformChanged(updateData) {
    pub fn overlay_transform_changed(uid: i32, data: types::OVRTransformUpdate);

    /// Called when the API has finished injecting into the browser and the API can now be used.
    #[wasm_bindgen(js_name = APIInit)]
    // TODO: check accordingly to reference.
    // reference: none
    pub fn api_init();

}

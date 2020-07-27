//! Events are functions you define that will be called by OVR Toolkit. When enabled, device updates will be called every frame.
//!
//! Some events are disabled by default, refer to the API section above for how to enable specific events.

use super::{types, wasm_bindgen};

#[wasm_bindgen]
extern "C" {

    /// Sends HMD and left/right controller position and rotation, also shows the active controller.
    /// (Needs to be enabled per overlay, refer to API above).
    pub fn DevicePositionUpdate(deviceInfo: types::OVRDeviceUpdate);

    /// Receives messages from other browser instances.
    pub fn ReceiveMessage(message: String);

    /// If the user is interacting with the current overlay.
    /// (Mouse over).
    pub fn InteractionStateChanged(isInteracting: bool);

    /// Called when an overlay is spawned.
    pub fn OverlayOpened(uid: i32);

    /// Called when an overlay is closed.
    pub fn OverlayClosed(uid: i32);

    /// Called when an overlay is moved or its size changes.
    /// (Needs to be enabled per overlay, refer to API above).
    pub fn OverlayTransformChanged(uid: i32, data: types::OVRTransformUpdate);

    /// Called when the API has finished injecting into the browser and the API can now be used.
    pub fn APIInit();

}

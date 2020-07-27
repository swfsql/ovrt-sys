//! Note: At this time any API call that returns a value needs a callback assigned to return the value to, the 'callback' value will just be the string name of a function.
//!
//! Any API call that returns a value also had an additional string optional parameter, this can be used to share data back to the callback function.

use super::{types, wasm_bindgen};

#[wasm_bindgen]
extern "C" {
    /// Spawn a new overlay.
    ///
    /// Returns an `uid`.
    pub fn SpawnOverlay(transformInfo: types::OVROverlayTransform, callback: String) -> i32;

    /// Set contents of an overlay.
    ///
    /// TODO: check what the following means:
    ///
    /// For website - `contents: OVRWebContents`.
    /// For desktop - `monitorId: i32`.
    /// For window - `windowHandle: i32`.
    pub fn SetContents(uid: i32, type_: i32);

    /// Returns a list of open windows and their handles.
    /// (If user has this option enabled).
    ///
    /// Returns `windowTitles`.
    pub fn GetWindowTitles(callback: String) -> types::KeyValuePairI32String;

    /// (Used for SetContents monitorId).
    ///
    /// Returns `monitorCount` (how many monitors are connected).
    pub fn GetMonitorCount(callback: String) -> i32;

    /// Refresh a browser page.
    pub fn Refresh(uid: i32);

    /// Get `OVROverlayTransform` of a specified overlay.
    ///
    /// Returns `transformInfo`.
    pub fn GetOverlayTransform(uid: i32, callback: String) -> types::OVROverlayTransform;

    /// Get type of overlay.
    /// (Browser, window capture, desktop capture).
    ///
    /// Returns `type`.
    pub fn GetOverlayType(uid: i32, callback: String) -> i32;

    /// Get bounds of overlay bounding box.
    /// (Refer to Unity documentation 'Bounds' section).
    ///
    /// Returns `bounds`.
    pub fn GetOverlayBounds(uid: i32, callback: String) -> types::OVROverlayBounds;

    /// Get finger curl positions.
    ///
    /// Returns `curls`.
    /// (Returns 0 for all values if user is in Simulator Mode).
    pub fn GetFingerCurls(callback: String) -> types::OVRFingerCurls;

    /// Set position of an overlay.
    pub fn SetOverlayPosition(uid: i32, posX: f64, posY: f64, posZ: f64);

    /// Set rotation of an overlay.
    /// (EulerAngles).
    pub fn SetOverlayRotation(uid: i32, rotX: f64, rotY: f64, rotZ: f64);

    /// Set overlay setting.
    ///
    /// TODO: check what the following means:
    ///
    /// newValue: i32/f64/bool
    pub fn SetOverlaySetting(uid: i32, setting: i32);

    /// Close the specified overlay.
    pub fn CloseOverlay(uid: i32);

    /// Send device position/rotation data to the calling overlay.
    /// (Refer to 'Events' below).
    pub fn SendDeviceData(enabled: bool);

    /// Send overlay position/rotation data to the calling overlay.
    /// (Refer to 'Events' below).
    pub fn SendOverlayPositions(enabled: bool);

    /// Send message all other open browser instances.
    /// (Calls 'ReceiveMessage').
    pub fn BroadcastMessage(message: String);

    /// Send message to specific browser instance.
    /// (Calls 'ReceiveMessage').
    pub fn SendMessage(uid: i32, message: String);

    /// Set if this overlay should receive keyboard inputs.
    /// (This will block the keyboard working on OVR Toolkit windows!).
    pub fn SetKeyboardFocus(enabled: bool);

    /// Sets the title of the browser that is visible in the Window List of OVR Toolkit.
    pub fn SetBrowserTitle(title: String);
}

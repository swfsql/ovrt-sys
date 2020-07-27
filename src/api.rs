//! Note: At this time any API call that returns a value needs a callback assigned to return the value to, the 'callback' value will just be the string name of a function.
//!
//! Any API call that returns a value also had an additional string optional parameter, this can be used to share data back to the callback function.

use super::{types, wasm_bindgen};

#[wasm_bindgen]
extern "C" {
    /// Spawn a new overlay.
    ///
    /// Returns an `uid`.
    #[wasm_bindgen(js_namespace = window, js_name = SpawnOverlay)]
    // TODO: check accordingly to reference.
    // reference: window.SpawnOverlay(JSON.stringify(transform), "ovrtWinSpawned");
    pub fn spawn_overlay(transformInfo: types::OVROverlayTransform, callback: String) -> i32;

    /// Set contents of an overlay.
    ///
    /// TODO: check what the following means:
    ///
    /// For website - `contents: OVRWebContents`.
    /// For desktop - `monitorId: i32`.
    /// For window - `windowHandle: i32`.
    #[wasm_bindgen(js_namespace = window, js_name = SetContents)]
    // TODO: check accordingly to reference.
    // reference: window.SetContents(String(uid), Number(winData.type), normalizedContents);
    pub fn set_contents(uid: i32, type_: i32);

    /// Returns a list of open windows and their handles.
    /// (If user has this option enabled).
    ///
    /// Returns `windowTitles`.
    #[wasm_bindgen(js_namespace = window, js_name = GetWindowTitles)]
    // TODO: check accordingly to reference.
    // reference: window.GetWindowTitles("completeIntervalWinTitles");
    // window.GetWindowTitles("ovrtWinTitles");
    pub fn get_window_titles(callback: String) -> types::KeyValuePairI32String;

    /// (Used for SetContents monitorId).
    ///
    /// Returns `monitorCount` (how many monitors are connected).
    #[wasm_bindgen(js_namespace = window, js_name = GetMonitorCount)]
    // TODO: check accordingly to reference.
    // reference: GetMonitorCount("ovrtMonitorTotal");
    // window.GetMonitorCount(callback, data);
    pub fn get_monitor_count(callback: String) -> i32;

    /// Refresh a browser page.
    #[wasm_bindgen(js_namespace = window, js_name = Refresh)]
    // TODO: check accordingly to reference.
    // reference: window.Refresh(uid);
    pub fn refresh(uid: i32);

    /// Get `OVROverlayTransform` of a specified overlay.
    ///
    /// Returns `transformInfo`.
    #[wasm_bindgen(js_namespace = window, js_name = GetOverlayTransform)]
    // TODO: check accordingly to reference.
    // reference: window.GetOverlayTransform(String(uid), "ovrtWinDetailed");
    pub fn get_overlay_transform(uid: i32, callback: String) -> types::OVROverlayTransform;

    /// Get type of overlay.
    /// (Browser, window capture, desktop capture).
    ///
    /// Returns `type`.
    #[wasm_bindgen(js_namespace = window, js_name = GetOverlayType)]
    // TODO: check accordingly to reference.
    // reference: window.GetOverlayType(uid, callback);
    pub fn get_overlay_type(uid: i32, callback: String) -> i32;

    /// Get bounds of overlay bounding box.
    /// (Refer to Unity documentation 'Bounds' section).
    ///
    /// Returns `bounds`.
    #[wasm_bindgen(js_namespace = window, js_name = GetOverlayBounds)]
    // TODO: check accordingly to reference.
    // reference: window.GetOverlayBounds(uid, callback);
    pub fn get_overlay_bounds(uid: i32, callback: String) -> types::OVROverlayBounds;

    /// Get finger curl positions.
    ///
    /// Returns `curls`.
    /// (Returns 0 for all values if user is in Simulator Mode).
    #[wasm_bindgen(js_namespace = window, js_name = GetFingerCurls)]
    // TODO: check accordingly to reference.
    // reference: window.GetFingerCurls("completeFingerCurls");
    pub fn get_finger_curls(callback: String) -> types::OVRFingerCurls;

    /// Set position of an overlay.
    #[wasm_bindgen(js_namespace = window, js_name = SetOverlayPosition)]
    // TODO: check accordingly to reference.
    // reference: window.SetOverlayPosition(uid, pos.x, pos.y, pos.z);
    pub fn set_overlay_position(uid: i32, posX: f64, posY: f64, posZ: f64);

    /// Set rotation of an overlay.
    /// (EulerAngles).
    #[wasm_bindgen(js_namespace = window, js_name = SetOverlayRotation)]
    // TODO: check accordingly to reference.
    // reference: window.SetOverlayRotation(uid, rot.x, rot.y, rot.z);
    pub fn set_overlay_rotation(uid: i32, rotX: f64, rotY: f64, rotZ: f64);

    /// Set overlay setting.
    ///
    /// TODO: check what the following means:
    ///
    /// newValue: i32/f64/bool
    #[wasm_bindgen(js_namespace = window, js_name = SetOverlaySetting)]
    // TODO: check accordingly to reference.
    // reference: window.SetOverlaySetting(uid, setting, value);
    pub fn set_overlay_setting(uid: i32, setting: i32);

    /// Close the specified overlay.
    #[wasm_bindgen(js_namespace = window, js_name = CloseOverlay)]
    // TODO: check accordingly to reference.
    // reference: window.CloseOverlay(uid);
    pub fn close_overlay(uid: i32);

    /// Send device position/rotation data to the calling overlay.
    /// (Refer to 'Events' below).
    #[wasm_bindgen(js_namespace = window, js_name = SendDeviceData)]
    // TODO: check accordingly to reference.
    // reference: window.SendDeviceData(enable);
    pub fn send_device_data(enabled: bool);

    /// Send overlay position/rotation data to the calling overlay.
    /// (Refer to 'Events' below).
    #[wasm_bindgen(js_namespace = window, js_name = SendOverlayPositions)]
    // TODO: check accordingly to reference.
    // reference: window.SendOverlayPositions(enable);
    pub fn send_overlay_positions(enabled: bool);

    /// Send message all other open browser instances.
    /// (Calls 'ReceiveMessage').
    #[wasm_bindgen(js_namespace = window, js_name = BroadcastMessage)]
    // TODO: check accordingly to reference.
    // reference: window.BroadcastMessage(JSON.stringify({broadcast: true,event: event,data: data,}));
    // window.BroadcastMessage(JSON.stringify({broadcast: false,event: event,data: data,senderId: senderId,targetId: targetId,}));
    //
    // check send_message below
    pub fn broadcast_message(message: String);

    /// Send message to specific browser instance.
    /// (Calls 'ReceiveMessage').
    #[wasm_bindgen(js_namespace = window, js_name = SendMessage)]
    // TODO: check accordingly to reference.
    // reference: none
    // TODO: actually, it appears that this was merged into broadcast_message
    // but now it has its own api call
    pub fn send_message(uid: i32, message: String);

    /// Set if this overlay should receive keyboard inputs.
    /// (This will block the keyboard working on OVR Toolkit windows!).
    #[wasm_bindgen(js_namespace = window, js_name = SetKeyboardFocus)]
    // TODO: check accordingly to reference.
    // reference: none
    pub fn set_keyboard_focus(enabled: bool);

    /// Sets the title of the browser that is visible in the Window List of OVR Toolkit.
    #[wasm_bindgen(js_namespace = window, js_name = SetBrowserTitle)]
    // TODO: check accordingly to reference.
    // reference: none
    pub fn set_browser_title(title: String);
}

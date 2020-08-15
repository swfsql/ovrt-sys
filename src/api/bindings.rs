//! Raw bindings into the javascript API.
//!
//! For using those functions, prefer `api` instead.

use crate::wasm_bindgen;

#[wasm_bindgen]
extern "C" {

    /// Spawn a new overlay.
    ///
    /// This is private/hidden for safety. See `spawn_overlay` for more info.
    ///
    /// Returns an uid.
    // https://github.com/swfsql/ovrt-sys/issues/2
    #[wasm_bindgen(js_namespace = window, js_name = SpawnOverlay)]
    pub(crate) fn spawn_overlay_with_callback(
        transform_info: String,
        callback: String,
    ) -> i32;

    /// Spawn a new overlay.
    ///
    /// This is private/hidden for safety. See `spawn_overlay` for more info.
    ///
    /// Returns an uid.
    #[wasm_bindgen(js_namespace = window, js_name = SpawnOverlay)]
    pub(crate) fn spawn_overlay(transform_info: String) -> i32;

    /// Set contents of an overlay.
    ///
    /// This is private/hidden for safety. See `types::WindowTypeValue` for more info.
    // https://github.com/swfsql/ovrt-sys/issues/2
    // https://github.com/swfsql/ovrt-sys/issues/6
    #[wasm_bindgen(js_namespace = window, js_name = SetContents)]
    pub(crate) fn set_contents_website(
        uid: String,
        type_: i32,
        contents: String,
    );

    /// Set contents of an overlay.
    ///
    /// This is private/hidden for safety. See `types::WindowTypeValue` for more info.
    #[wasm_bindgen(js_namespace = window, js_name = SetContents)]
    // TODO: check accordingly to reference.
    // reference: window.SetContents(String(uid), Number(winData.type), normalizedContents);
    // https://github.com/swfsql/ovrt-sys/issues/6
    pub(crate) fn set_contents_desktop(
        uid: String,
        type_: i32,
        monitor_id: i32,
    );

    /// Set contents of an overlay.
    ///
    /// This is private/hidden for safety. See `types::WindowTypeValue` for more info.
    #[wasm_bindgen(js_namespace = window, js_name = SetContents)]
    // TODO: check accordingly to reference.
    // reference: window.SetContents(String(uid), Number(winData.type), normalizedContents);
    // https://github.com/swfsql/ovrt-sys/issues/6
    pub(crate) fn set_contents_window(
        uid: String,
        type_: i32,
        window_handle: i32,
    );

    /// Returns a list of open windows and their handles.
    /// (If user has this option enabled).
    ///
    /// Returns `windowTitles`.
    // https://github.com/swfsql/ovrt-sys/issues/2
    #[wasm_bindgen(js_namespace = window, js_name = GetWindowTitles)]
    // TODO: check accordingly to reference.
    // reference: window.GetWindowTitles("completeIntervalWinTitles");
    // window.GetWindowTitles("ovrtWinTitles");
    pub(crate) fn get_window_titles(callback: String) -> String;

    /// Returns a list of open windows and their handles.
    /// (If user has this option enabled).
    ///
    /// Returns `windowTitles`.
    #[wasm_bindgen(js_name = GetWindowTitlesCallback)]
    // TODO: check accordingly to reference.
    // reference: window.GetWindowTitles("completeIntervalWinTitles");
    // window.GetWindowTitles("ovrtWinTitles");
    pub(crate) fn get_window_titles_callback(
        callback: String,
    ) -> String;

    /// (Used for SetContents monitorId).
    ///
    /// Returns `monitorCount` (how many monitors are connected).
    #[wasm_bindgen(js_namespace = window, js_name = GetMonitorCount)]
    // TODO: check accordingly to reference.
    // reference: GetMonitorCount("ovrtMonitorTotal");
    // window.GetMonitorCount(callback, data);
    pub(crate) fn get_monitor_count(callback: String) -> i32;

    /// Refresh a browser page.
    #[wasm_bindgen(js_namespace = window, js_name = Refresh)]
    // TODO: check accordingly to reference.
    // reference: window.Refresh(uid);
    pub(crate) fn refresh(uid: i32);

    /// Get `OVROverlayTransform` of a specified overlay.
    ///
    /// Returns `transformInfo`.
    // https://github.com/swfsql/ovrt-sys/issues/2
    #[wasm_bindgen(js_namespace = window, js_name = GetOverlayTransform)]
    // TODO: check accordingly to reference.
    // reference: window.GetOverlayTransform(String(uid), "ovrtWinDetailed");
    pub(crate) fn get_overlay_transform(uid: i32) -> String;

    /// Get `OVROverlayTransform` of a specified overlay.
    ///
    /// Returns `transformInfo`.
    #[wasm_bindgen(js_namespace = window, js_name = GetOverlayTransform)]
    // TODO: check accordingly to reference.
    // reference: window.GetOverlayTransform(String(uid), "ovrtWinDetailed");
    pub(crate) fn get_overlay_transform_with_callback(
        uid: i32,
        callback: String,
    ) -> String;

    /// Get type of overlay.
    /// (Browser, window capture, desktop capture).
    ///
    /// Returns `type`.
    #[wasm_bindgen(js_namespace = window, js_name = GetOverlayType)]
    // TODO: check accordingly to reference.
    // reference: window.GetOverlayType(uid, callback);
    pub(crate) fn get_overlay_type(
        uid: i32,
        callback: String,
    ) -> i32;

    /// Get bounds of overlay bounding box.
    /// (Refer to Unity documentation 'Bounds' section).
    ///
    /// Returns `bounds`.
    // https://github.com/swfsql/ovrt-sys/issues/2
    #[wasm_bindgen(js_namespace = window, js_name = GetOverlayBounds)]
    // TODO: check accordingly to reference.
    // reference: window.GetOverlayBounds(uid, callback);
    pub(crate) fn get_overlay_bounds(
        uid: i32,
        callback: String,
    ) -> String;

    /// Get finger curl positions.
    ///
    /// Returns `curls`.
    /// (Returns 0 for all values if user is in Simulator Mode).
    // https://github.com/swfsql/ovrt-sys/issues/2
    #[wasm_bindgen(js_namespace = window, js_name = GetFingerCurls)]
    // TODO: check accordingly to reference.
    // reference: window.GetFingerCurls("completeFingerCurls");
    pub(crate) fn get_finger_curls(callback: String) -> String;

    /// Set position of an overlay.
    #[wasm_bindgen(js_namespace = window, js_name = SetOverlayPosition)]
    // TODO: check accordingly to reference.
    // reference: window.SetOverlayPosition(uid, pos.x, pos.y, pos.z);
    pub(crate) fn set_overlay_position(
        uid: i32,
        posX: f64,
        posY: f64,
        posZ: f64,
    );

    /// Set rotation of an overlay.
    /// (EulerAngles).
    #[wasm_bindgen(js_namespace = window, js_name = SetOverlayRotation)]
    // TODO: check accordingly to reference.
    // reference: window.SetOverlayRotation(uid, rot.x, rot.y, rot.z);
    pub(crate) fn set_overlay_rotation(
        uid: i32,
        rotX: f64,
        rotY: f64,
        rotZ: f64,
    );

    /// Set overlay setting.
    ///
    /// This is private/hidden for safety. See `types::SettingValue` for more info.
    #[wasm_bindgen(js_namespace = window, js_name = SetOverlaySetting)]
    // TODO: check accordingly to reference.
    // reference: window.SetOverlaySetting(uid, setting, value);
    pub(crate) fn set_overlay_setting_i32(
        uid: i32,
        setting: i32,
        new_value: i32,
    );

    /// Set overlay setting.
    ///
    /// This is private/hidden for safety. See `types::SettingValue` for more info.
    #[wasm_bindgen(js_namespace = window, js_name = SetOverlaySetting)]
    pub(crate) fn set_overlay_setting_f64(
        uid: i32,
        setting: i32,
        new_value: f64,
    );

    /// Set overlay setting.
    ///
    /// This is private/hidden for safety. See `types::SettingValue` for more info.
    #[wasm_bindgen(js_namespace = window, js_name = SetOverlaySetting)]
    pub(crate) fn set_overlay_setting_bool(
        uid: i32,
        setting: i32,
        new_value: bool,
    );

    // /// Close the specified overlay.
    // #[wasm_bindgen(js_namespace = window, js_name = CloseOverlay)]
    // // TODO: check accordingly to reference.
    // // reference: window.CloseOverlay(uid);
    // // https://github.com/swfsql/ovrt-sys/issues/6
    // pub(crate) fn close_overlay(uid: i32);

    /// Close the specified overlay.
    #[wasm_bindgen(js_namespace = window, js_name = CloseOverlay)]
    // TODO: check accordingly to reference.
    // reference: window.CloseOverlay(uid);
    // https://github.com/swfsql/ovrt-sys/issues/6
    // https://github.com/swfsql/ovrt-sys/issues/2
    pub(crate) fn close_overlay_str(uid: String);

    /// Send device position/rotation data to the calling overlay.
    /// (Refer to 'Events' below).
    #[wasm_bindgen(js_namespace = window, js_name = SendDeviceData)]
    // TODO: check accordingly to reference.
    // reference: window.SendDeviceData(enable);
    pub(crate) fn send_device_data(enabled: bool);

    /// Send overlay position/rotation data to the calling overlay.
    /// (Refer to 'Events' below).
    #[wasm_bindgen(js_namespace = window, js_name = SendOverlayPositions)]
    // TODO: check accordingly to reference.
    // reference: window.SendOverlayPositions(enable);
    pub(crate) fn send_overlay_positions(enabled: bool);

    /// Send message all other open browser instances.
    /// (Calls 'ReceiveMessage').
    #[wasm_bindgen(js_namespace = window, js_name = BroadcastMessage)]
    // TODO: check accordingly to reference.
    // reference: window.BroadcastMessage(JSON.stringify({broadcast: true,event: event,data: data,}));
    // window.BroadcastMessage(JSON.stringify({broadcast: false,event: event,data: data,senderId: senderId,targetId: targetId,}));
    //
    // check send_message below
    pub(crate) fn broadcast_message(message: String);

    /// Send message to specific browser instance.
    /// (Calls 'ReceiveMessage').
    #[wasm_bindgen(js_namespace = window, js_name = SendMessage)]
    // TODO: check accordingly to reference.
    // reference: none
    // TODO: actually, it appears that this was merged into broadcast_message
    // but now it has its own api call
    pub(crate) fn send_message(uid: i32, message: String);

    /// Set if this overlay should receive keyboard inputs.
    /// (This will block the keyboard working on OVR Toolkit windows!).
    #[wasm_bindgen(js_namespace = window, js_name = SetKeyboardFocus)]
    // TODO: check accordingly to reference.
    // reference: none
    pub(crate) fn set_keyboard_focus(enabled: bool);

    /// Sets the title of the browser that is visible in the Window List of OVR Toolkit.
    #[wasm_bindgen(js_namespace = window, js_name = SetBrowserTitle)]
    // TODO: check accordingly to reference.
    // reference: none
    pub(crate) fn set_browser_title(title: String);
}

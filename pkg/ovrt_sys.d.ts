/* tslint:disable */
/* eslint-disable */
/**
*/
export function wasm_main(): void;
/**
* Sends HMD and left/right controller position and rotation, also shows the active controller.
* (Needs to be enabled per overlay, refer to API above).
* @param {string} device_info
*/
export function DevicePositionUpdate(device_info: string): void;
/**
* Receives messages from other browser instances.
* @param {string} message
*/
export function ReceiveMessage(message: string): void;
/**
* If the user is interacting with the current overlay.
* (Mouse over).
* @param {boolean} is_interacting
*/
export function InteractionStateChanged(is_interacting: boolean): void;
/**
* Called when an overlay is spawned.
* @param {number} uid
*/
export function OverlayOpened(uid: number): void;
/**
* Called when an overlay is closed.
* @param {number} uid
*/
export function OverlayClosed(uid: number): void;
/**
* Called when an overlay is moved or its size changes.
* (Needs to be enabled per overlay, refer to API above).
* @param {number} uid
* @param {string} data
*/
export function OverlayTransformChanged(uid: number, data: string): void;
/**
* Called when the API has finished injecting into the browser and the API can now be used.
*/
export function APIInit(): void;
/**
* Spawn a new overlay.
*
* This is private/hidden for safety. See `spawn_overlay` for more info.
*
* Returns an uid.
* @param {number} uid
*/
export function SpawnOverlayOvrtSysCallback(uid: number): void;
/**
* Returns a list of open windows and their handles.
* (If user has this option enabled).
*
* Returns `windowTitles`.
* @param {string} window_titles
*/
export function GetWindowTitlesOvrtSysCallback(window_titles: string): void;
/**
* (Used for SetContents monitorId).
*
* Returns `monitorCount` (how many monitors are connected).
* @param {number} monitor_count
*/
export function GetMonitorCountOvrtSysCallback(monitor_count: number): void;
/**
* Get `OVROverlayTransform` of a specified overlay.
*
* Returns `transformInfo`.
* @param {string} transform_info
*/
export function GetOverlayTransformOvrtSysCallback(transform_info: string): void;
/**
* Get type of overlay.
* (Browser, window capture, desktop capture).
*
* Returns `type`.
* @param {number} type_
*/
export function GetOverlayTypeOvrtSysCallback(type_: number): void;
/**
* Get bounds of overlay bounding box.
* (Refer to Unity documentation 'Bounds' section).
*
* Returns `bounds`.
* @param {string} bounds
*/
export function GetOverlayBoundsOvrtSysCallback(bounds: string): void;
/**
* Get finger curl positions.
*
* Returns `curls`.
* (Returns 0 for all values if user is in Simulator Mode).
* @param {string} curls
*/
export function GetFingerCurlsOvrtSysCallback(curls: string): void;
/**
*/
export enum Device {
/**
* None/World.
*/
  World,
  Hmd,
  RightHand,
  LeftHand,
}
/**
* Represents kinds of window types.
*
* See also: `types::WindowTypeValue`.
*/
export enum WindowType {
  WebPage,
  DesktopCapture,
  WindowCapture,
}
/**
* Represents kinds of setting values.
*
* See also: `types::SettingValue`.
*/
export enum Setting {
/**
* (Width in meters) (f64).
*/
  Size,
/**
* (f64).
*/
  Opacity,
/**
* (f64).
*/
  Curvature,
/**
* (i32).
*/
  Framerate,
/**
* (bool).
*/
  EcoMode,
/**
* (bool).
*/
  LookHiding,
/**
* (i32).
*/
  AttachedDevice,
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly wasm_main: () => void;
  readonly DevicePositionUpdate: (a: number, b: number) => void;
  readonly ReceiveMessage: (a: number, b: number) => void;
  readonly InteractionStateChanged: (a: number) => void;
  readonly OverlayOpened: (a: number) => void;
  readonly OverlayClosed: (a: number) => void;
  readonly OverlayTransformChanged: (a: number, b: number, c: number) => void;
  readonly APIInit: () => void;
  readonly SpawnOverlayOvrtSysCallback: (a: number) => void;
  readonly GetWindowTitlesOvrtSysCallback: (a: number, b: number) => void;
  readonly GetMonitorCountOvrtSysCallback: (a: number) => void;
  readonly GetOverlayTransformOvrtSysCallback: (a: number, b: number) => void;
  readonly GetOverlayTypeOvrtSysCallback: (a: number) => void;
  readonly GetOverlayBoundsOvrtSysCallback: (a: number, b: number) => void;
  readonly GetFingerCurlsOvrtSysCallback: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h29f8c3340cfc6fb4: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hdce51312cfc1b2f1: (a: number, b: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        
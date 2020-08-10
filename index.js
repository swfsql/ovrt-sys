import init, {
  // events
  DevicePositionUpdate,
  ReceiveMessage,
  InteractionStateChanged,
  OverlayOpened,
  OverlayClosed,
  OverlayTransformChanged,
  APIInit,
  // callbacks
  SpawnOverlayOvrtSysCallback,
  GetWindowTitlesOvrtSysCallback,
  GetMonitorCountOvrtSysCallback,
  GetOverlayTransformOvrtSysCallback,
  GetOverlayTypeOvrtSysCallback,
  GetOverlayBoundsOvrtSysCallback,
  GetFingerCurlsOvrtSysCallback,
  //
  wasm_main,
} from "./pkg/ovrt_sys.js";

export {
  // events
  DevicePositionUpdate,
  ReceiveMessage,
  InteractionStateChanged,
  OverlayOpened,
  OverlayClosed,
  OverlayTransformChanged,
  APIInit,
  // callbacks
  SpawnOverlayOvrtSysCallback,
  GetWindowTitlesOvrtSysCallback,
  GetMonitorCountOvrtSysCallback,
  GetOverlayTransformOvrtSysCallback,
  GetOverlayTypeOvrtSysCallback,
  GetOverlayBoundsOvrtSysCallback,
  GetFingerCurlsOvrtSysCallback,
};

async function run() {
  await init();

  console.trace("creating workaroud");

  // workaroud to export module items
  // https://github.com/swfsql/ovrt-sys/issues/4
  window.ovrt_sys = {
    // events
    DevicePositionUpdate: DevicePositionUpdate,
    ReceiveMessage: ReceiveMessage,
    InteractionStateChanged: InteractionStateChanged,
    OverlayOpened: OverlayOpened,
    OverlayClosed: OverlayClosed,
    OverlayTransformChanged: OverlayTransformChanged,
    APIInit: APIInit,
    // callbacks
    SpawnOverlayOvrtSysCallback: SpawnOverlayOvrtSysCallback,
    GetWindowTitlesOvrtSysCallback: GetWindowTitlesOvrtSysCallback,
    GetMonitorCountOvrtSysCallback: GetMonitorCountOvrtSysCallback,
    GetOverlayTransformOvrtSysCallback: GetOverlayTransformOvrtSysCallback,
    GetOverlayTypeOvrtSysCallback: GetOverlayTypeOvrtSysCallback,
    GetOverlayBoundsOvrtSysCallback: GetOverlayBoundsOvrtSysCallback,
    GetFingerCurlsOvrtSysCallback: GetFingerCurlsOvrtSysCallback,
  };
  // https://github.com/swfsql/ovrt-sys/issues/4
  ovrt_sys_use_workaroud();

  wasm_main();
}
run();

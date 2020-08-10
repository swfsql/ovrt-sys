// https://github.com/swfsql/ovrt-sys/issues/4
function ovrt_sys_use_workaroud() {
  // workaroud to import module items
  console.trace("re-exporting event callbacks into the global scope");

  // events
  DevicePositionUpdate = window.ovrt_sys.DevicePositionUpdate;
  ReceiveMessage = window.ovrt_sys.ReceiveMessage;
  InteractionStateChanged = window.ovrt_sys.InteractionStateChanged;
  OverlayOpened = window.ovrt_sys.OverlayOpened;
  OverlayClosed = window.ovrt_sys.OverlayClosed;
  OverlayTransformChanged = window.ovrt_sys.OverlayTransformChanged;
  APIInit = window.ovrt_sys.APIInit;
  // callbacks
  SpawnOverlayOvrtSysCallback = window.ovrt_sys.SpawnOverlayOvrtSysCallback;
  GetWindowTitlesOvrtSysCallback =
    window.ovrt_sys.GetWindowTitlesOvrtSysCallback;
  GetMonitorCountOvrtSysCallback =
    window.ovrt_sys.GetMonitorCountOvrtSysCallback;
  GetOverlayTransformOvrtSysCallback =
    window.ovrt_sys.GetOverlayTransformOvrtSysCallback;
  GetOverlayTypeOvrtSysCallback = window.ovrt_sys.GetOverlayTypeOvrtSysCallback;
  GetOverlayBoundsOvrtSysCallback =
    window.ovrt_sys.GetOverlayBoundsOvrtSysCallback;
  GetFingerCurlsOvrtSysCallback = window.ovrt_sys.GetFingerCurlsOvrtSysCallback;
}

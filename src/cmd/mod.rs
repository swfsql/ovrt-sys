pub mod sink;

use crate::types;

/// Notifications only exists in ovrt-sys and may be used to better
/// control the app. Everytime a request is made for ovrt, a
/// Notification is also sent by ovrt-sys, feedbacking itself.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Notification {
    // https://github.com/swfsql/ovrt-sys/issues/5
    /// See also Callback::FinishSpawnOverlay
    /// and EventResponse::FinishSpawnOverlay.
    StartSpawnOverlay,
    /// See also EventResponse::FinishCloseOverlay.
    StartCloseOverlay(types::Uid),
    /// See also Callback::FinishGetWindowTitles.
    StartGetWindowTitles,
    /// See also Callback::FinishGetMonitorCount.
    StartGetMonitorCount,
    /// See also Callback::FinishGetOverlayTransform.
    StartGetOverlayTransform(types::Uid),
    /// See also Callback::FinishGetOverlayType.
    StartGetOverlayType(types::Uid),
    /// See also Callback::FinishGetOverlayBounds.
    StartGetOverlayBounds(types::Uid),
    /// See also Callback::FinishGetFingerCurls.
    StartGetFingerCurls,
}

/// Callback are pre-registered functions by ovrt-sys in which
/// ovrt calls after some job/query has finished.
///
/// Every callback is called as a response from a request made by
/// ovrt-sys, and thus for each Callback instance, there was
/// a Notification instance as well.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Callback {
    /// See also Notification::StartSpawnOverlay.
    FinishSpawnOverlay(types::Uid),
    /// See also Notification::StartGetWindowTitles.
    FinishGetWindowTitles(String),
    /// See also Notification::StartGetMonitorCount.
    FinishGetMonitorCount(usize),
    // TODO: try getting the uid as well, or it could be ambiguous
    /// See also Notification::StartGetOverlayTransform.
    FinishGetOverlayTransform(types::OVROverlayTransform),
    /// See also Notification::StartGetOverlayType.
    FinishGetOverlayType(i32),
    /// See also Notification::StartGetOverlayBounds.
    FinishGetOverlayBounds(String),
    /// See also Notification::StartGetFingerCurls.
    FinishGetFingerCurls(String),
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum EventResponse {
    /// See also Notification::StartSpawnOverlay.
    FinishSpawnOverlay(types::Uid),
    /// See also Notification::StartCloseOverlay.
    FinishCloseOverlay(types::Uid),
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Event {
    DevicePositionUpdate(types::OVRDeviceUpdate),
    ReceiveMessage(String),
    InteractionStateChanged {
        is_interacting: bool,
    },
    ApiInit,
    OverlayTransformChanged(
        types::Uid,
        types::OVRTransformUpdate,
    ),
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Command {
    Callback(Callback),
    /// Events are pre-registered functions by ovrt-sys in which
    /// ovrt calls at anytime.
    ///
    /// Those events are not responses of calls made by ovrt-sys,
    /// and thus they are not related to Callbacks nor Notifications.
    ///
    /// See EventResponse for another kind of events.
    Event(Event),
    /// EventResponse are pre-registered functions by ovrt-sys in which
    /// ovrt calls when some query/job has finished.
    ///
    /// Those events are responses of calls made by ovrt-sys,
    /// and thus they are related to Notifications, or possibly even
    /// Callbacks.
    ///
    /// See Event for another kind of events.
    EventResponse(EventResponse),
    Notification(Notification),
}

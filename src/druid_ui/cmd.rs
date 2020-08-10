// use crate::log;
// use druid::im::Vector;
// use druid::widget::{Button, Flex};
// use druid::{
//     AppLauncher, Data, ExtEventSink, Lens, LocalizedString, Widget, WidgetExt, WindowDesc,
// };

use crate::types;
use druid::Selector;

// pub const TEST_CMD: Selector<types::Uid> = Selector::new("test_cmd");
// pub const START_SLOW_FUNCTION: Selector<u32> = Selector::new("start_slow_function");
// pub const FINISH_SLOW_FUNCTION: Selector<u32> = Selector::new("finish_slow_function");

// pub const START_SPAWN_OVERLAY: Selector<()> = Selector::new("start_spawn_overlay");
pub const FINISH_SPAWN_OVERLAY_EVENT: Selector<types::Uid> =
    Selector::new("finish_spawn_overlay_event");
pub const FINISH_SPAWN_OVERLAY_CALLBACK: Selector<types::Uid> =
    Selector::new("finish_spawn_overlay_callback");
//
// pub const START_CLOSE_OVERLAY: Selector<()> = Selector::new("start_close_overlay");
pub const FINISH_CLOSE_OVERLAY_EVENT: Selector<types::Uid> =
    Selector::new("finish_close_overlay_event");
//
// pub const START_GET_WINDOW_TITLES: Selector<()> = Selector::new("start_get_window_titles");
pub const FINISH_GET_WINDOW_TITLES_CALLBACK: Selector<String> =
    Selector::new("finish_get_window_titles_callback");
//
// pub const START_GET_MONITOR_COUNT: Selector<()> = Selector::new("start_get_monitor_count");
pub const FINISH_GET_MONITOR_COUNT_CALLBACK: Selector<usize> =
    Selector::new("finish_get_monitor_count_callback");
//
// pub const START_GET_OVERLAY_TRANSFORM: Selector<()> = Selector::new("start_get_overlay_transform");
pub const FINISH_GET_OVERLAY_TRANSFORM_CALLBACK: Selector<types::OVROverlayTransform> =
    Selector::new("finish_get_overlay_transform_callback");
//
// pub const START_GET_OVERLAY_TYPE: Selector<()> = Selector::new("start_get_overlay_type");
pub const FINISH_GET_OVERLAY_TYPE_CALLBACK: Selector<i32> =
    Selector::new("finish_get_overlay_type_callback");
//
// pub const START_GET_OVERLAY_BOUNDS: Selector<()> = Selector::new("start_get_overlay_bounds");
pub const FINISH_GET_OVERLAY_BOUNDS_CALLBACK: Selector<String> =
    Selector::new("finish_get_overlay_bounds_callback");
//
// pub const START_GET_FINGER_CURLS: Selector<()> = Selector::new("start_get_finger_curls");
pub const FINISH_GET_FINGER_CURLS_CALLBACK: Selector<String> =
    Selector::new("finish_get_finger_curls_callback");
//

pub const EVENT_DEVICE_POSITION_UPDATE: Selector<types::OVRDeviceUpdate> =
    Selector::new("event_device_position_update");
pub const EVENT_RECEIVE_MESSAGE: Selector<String> = Selector::new("event_receive_message");
pub const EVENT_INTERACTION_STATE_CHANGED: Selector<bool> =
    Selector::new("event_interaction_state_changed");
pub const EVENT_API_INIT: Selector<()> = Selector::new("event_api_init");
pub const EVENT_OVERLAY_TRANSFORM_CHANGED: Selector<(types::Uid, types::OVRTransformUpdate)> =
    Selector::new("event_overlay_transform_changed");

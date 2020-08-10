pub mod cmd;

use super::{api, types};
use crate::{log, v};
use druid::im::Vector;
use druid::widget::{Button, Flex};
use druid::{
    AppLauncher, Data, ExtEventSink, Lens, LocalizedString, Widget, WidgetExt, WindowDesc,
};
// use druid::widget::{Label, List, Scroll};
// use druid::lens::{self, LensExt};

use once_cell::sync::OnceCell;

pub struct Delegate {
    // unneeded field since this will be static, but anyway..
    pub eventsink: ExtEventSink,
}

#[derive(Debug, Clone, Default, Data, Lens)]
pub struct AppData {
    overlays: Vector<types::Uid>,
}

pub fn global_event_sink(app_launcher: Option<&AppLauncher<AppData>>) -> &'static ExtEventSink {
    // every clone on this instance by the users is unneeded, but anyway..
    static EVENT_SINK: OnceCell<ExtEventSink> = OnceCell::new();
    EVENT_SINK.get_or_init(|| {
        let app_launcher = app_launcher
            .expect("a running app_launcher is required when initializing the event_sink.");
        app_launcher.get_external_handle()
    })
}

pub fn run() {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("app-window-title").with_placeholder("App Window Title"));
    // Set our initial data
    let data = AppData::default();
    let app = AppLauncher::with_window(main_window);
    let delegate = Delegate {
        eventsink: global_event_sink(Some(&app)).clone(),
    };
    app.delegate(delegate)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

impl druid::AppDelegate<AppData> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        _data: &mut AppData,
        _env: &druid::Env,
    ) -> bool {
        if let Some(uid) = cmd
            .get(cmd::FINISH_SPAWN_OVERLAY_EVENT)
            .or_else(|| cmd.get(cmd::FINISH_SPAWN_OVERLAY_CALLBACK))
        {
            log!(v(&uid));
        } else if let Some(uid) = cmd.get(cmd::FINISH_CLOSE_OVERLAY_EVENT) {
            log!(v(&uid));
        } else if let Some(win_titles) = cmd.get(cmd::FINISH_GET_WINDOW_TITLES_CALLBACK) {
            log!(win_titles);
        } else if let Some(monitor_count) = cmd.get(cmd::FINISH_GET_MONITOR_COUNT_CALLBACK) {
            log!(v(&monitor_count));
        } else if let Some(overlay_transform) = cmd.get(cmd::FINISH_GET_OVERLAY_TRANSFORM_CALLBACK)
        {
            log!(v(&overlay_transform));
        } else if let Some(overlay_type) = cmd.get(cmd::FINISH_GET_OVERLAY_TYPE_CALLBACK) {
            log!(v(&overlay_type));
        } else if let Some(overlay_bounds) = cmd.get(cmd::FINISH_GET_OVERLAY_BOUNDS_CALLBACK) {
            log!(overlay_bounds);
        } else if let Some(finger_curls) = cmd.get(cmd::FINISH_GET_FINGER_CURLS_CALLBACK) {
            log!(finger_curls);
        } else if let Some(device_pos) = cmd.get(cmd::EVENT_DEVICE_POSITION_UPDATE) {
            log!(v(&device_pos));
        } else if let Some(msg) = cmd.get(cmd::EVENT_RECEIVE_MESSAGE) {
            log!(msg);
        } else if let Some(is_interacting) = cmd.get(cmd::EVENT_INTERACTION_STATE_CHANGED) {
            log!(v(&is_interacting));
        } else if let Some(()) = cmd.get(cmd::EVENT_API_INIT) {
            log!();
        } else if let Some((uid, overlay_transform_update)) =
            cmd.get(cmd::EVENT_OVERLAY_TRANSFORM_CHANGED)
        {
            log!(v(&uid), v(&overlay_transform_update));
        }
        true
    }
}

fn ui_builder() -> impl Widget<AppData> {
    let mut root = Flex::column();

    root.add_child(
        Button::new("Add Overlay")
            .on_click(|_ctx, _data: &mut AppData, _| {
                log!("calling spawn_overlay");
                let uid = api::spawn_overlay(&Default::default());
                log!("called spawn_overlay", v(&uid));
            })
            .fix_height(30.0)
            .expand_width(),
    );

    // Mark the widget as needing its layout rects painted
    // root.debug_paint_layout()
    root
}

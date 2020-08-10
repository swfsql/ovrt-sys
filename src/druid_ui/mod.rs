pub mod cmd;

use super::{api, types};
use crate::{log, v};
use druid::im::Vector;
use druid::widget::{Button, Flex};
use druid::widget::{Either, Label, List, Scroll};
use druid::{
    AppLauncher, Data, ExtEventSink, Lens, LocalizedString, UnitPoint, Widget, WidgetExt,
    WindowDesc,
};
use once_cell::sync::OnceCell;

// use druid::lens::{self, LensExt};

pub struct Delegate {
    // unneeded field since this will be static, but anyway..
    pub eventsink: ExtEventSink,
}

#[derive(Debug, Clone, Default, Data, Lens)]
pub struct AppData {
    overlays: Vector<Overlay>,
}

#[derive(Debug, Clone, Data, PartialOrd, Ord, PartialEq, Eq)]
pub enum Overlay {
    Spawning,
    Live(types::Uid),
    Closing(types::Uid),
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
        data: &mut AppData,
        _env: &druid::Env,
    ) -> bool {
        if let Some(uid) = cmd
            .get(cmd::FINISH_SPAWN_OVERLAY_EVENT)
            .or_else(|| cmd.get(cmd::FINISH_SPAWN_OVERLAY_CALLBACK))
        {
            let ov = data
                .overlays
                .back_mut()
                .expect("expecting some overlay item");
            // the last overlay should be an spawning one
            // otherwise something must have gone wrong
            assert_eq!(Overlay::Spawning, *ov);
            *ov = Overlay::Live(uid.clone());
        } else if let Some(uid) = cmd.get(cmd::FINISH_CLOSE_OVERLAY_EVENT) {
            let (i, _uid) = data
                .overlays
                .iter()
                .enumerate()
                .filter_map(|(i, ov)| match ov {
                    Overlay::Closing(inner_uid) => Some((i, inner_uid)),
                    _ => None,
                })
                .find(|(_i, inner_uid)| *inner_uid == uid)
                .expect("could not find the uid of the removed overlay");
            data.overlays.remove(i);
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
            .on_click(|_ctx, data: &mut AppData, _| {
                data.overlays.push_back(Overlay::Spawning);
                // https://github.com/swfsql/ovrt-sys/issues/5
                let _zero_uid = api::spawn_overlay(&Default::default());
            })
            .fix_height(30.0)
            .expand_width(),
    );

    let list = List::new(|| {
        Flex::row()
            .with_child(
                Label::new(|ov: &Overlay, _env: &_| {
                    use Overlay::*;
                    match ov {
                        Spawning => "Spawning..".to_string(),
                        Live(uid) => format!("Overlay #{}", uid.0),
                        Closing(uid) => format!("Closing #{}..", uid.0),
                    }
                }), // .align_vertical(UnitPoint::LEFT),
            )
            .with_flex_spacer(1.0)
            .with_child(Either::new(
                |ov: &Overlay, _env| {
                    if let Overlay::Live(_) = ov {
                        true
                    } else {
                        false
                    }
                },
                Button::new("Delete")
                    .on_click(|_ctx, ov: &mut Overlay, _env| {
                        if let Overlay::Live(uid) = ov {
                            let uid = uid.clone();
                            *ov = Overlay::Closing(uid.clone());
                            api::close_overlay(uid);
                        } else {
                            unreachable!("Either should have hidden this path");
                        }
                    })
                    .fix_size(80.0, 20.0)
                    .align_vertical(UnitPoint::CENTER),
                Label::new("Delete"),
            ))
        // .padding(10.0)
        // .background(Color::rgb(0.5, 0.0, 0.5))
        // .fix_height(50.0)
    });
    let list = Scroll::new(list)
        .vertical()
        .lens(AppData::overlays)
        // .lens(lens::Id.map(
        //     // Expose shared data with children data
        //     |d: &AppData| (d.right.clone(), d.right.clone()),
        //     |d: &mut AppData, x: (Vector<u32>, Vector<u32>)| {
        //         // If shared data was changed reflect the changes in our AppData
        //         d.right = x.0
        //     },
        // )),
        ;
    let mut flex_list = Flex::row();
    flex_list.add_flex_child(list, 1.0);

    root.add_flex_child(flex_list, 1.0);

    // Mark the widget as needing its layout rects painted
    // root.debug_paint_layout()
    root
}

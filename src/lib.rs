//! Bindings for ovr-toolkit custom apps js api.
//!
//! Based on [wiki/CustomApps](http://wiki.ovrtoolkit.co.uk/index.php?title=CustomApps&oldid=390)
//! and on [Zetaphor/ovrt-helper](https://github.com/Zetaphor/ovrt-helper/blob/524815728c19d431431f2f7a9a6e3ca7a6a19691/ovrt-helper.js).

// TODO: check returned UID
// TODO: check more functions,
// maybe those related to that created overlay
// TODO: insert some druid elements,
// such as "click to add overlay" stuff
// TODO: visualize and change window properties
// TODO: try changing any window property

#![allow(unused_unsafe)]

pub mod api;
pub mod consts;
pub mod events;
pub mod log;
pub mod types;

pub(crate) use log::js_value as v;
use wasm_bindgen::prelude::wasm_bindgen;

// This wrapper function is the primary modification we're making to the vanilla
// hello.rs example.
#[wasm_bindgen]
pub fn wasm_main() {
    // This hook is necessary to get panic messages in the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main()
}

#[wasm_bindgen]
pub fn spawn_overlay_callback(uid: i32) {
    log!("spawn_overlay_callback.", "Uid:", uid);
}

pub fn main() {
    log!("app started");
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("app-window-title").with_placeholder("App Window Title"));
    // Set our initial data
    let data = AppData::default();
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
    log!("app finished");

    // ovrt.createWebWin(
    //     `https://www.twitch.tv/popout/${username}/chat?popout=`,
    //     400,
    //     500
    //   );
}

use druid::im::Vector;
// use druid::lens::{self, LensExt};
use druid::widget::{Button, Flex};
// use druid::widget::{Label, List, Scroll};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

#[derive(Debug, Clone, Default, Data, Lens)]
struct AppData {
    overlays: Vector<types::Uid>,
}

fn ui_builder() -> impl Widget<AppData> {
    let mut root = Flex::column();

    root.add_child(
        Button::new("Add Overlay")
            .on_click(|_ctx, _data: &mut AppData, _| {
                log!("calling spawn_overlay");
                let uid = api::spawn_overlay(&Default::default(), "spawn_overlay_callback".into());
                log!("called spawn_overlay", "Uid:", uid.0);
            })
            .fix_height(30.0)
            .expand_width(),
    );

    // Mark the widget as needing its layout rects painted
    // root.debug_paint_layout()
    root
}

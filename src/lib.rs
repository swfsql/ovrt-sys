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

#[cfg(feature = "druid")]
pub mod druid_ui;

#[cfg(feature = "druid")]
pub use druid_ui as ui;

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

#[cfg(feature = "druid")]
pub fn main() {
    log!("app started");
    ui::run();
    log!("app finished starting");
}

#[cfg(not(feature = "druid"))]
pub fn main() {
    log!("app started");
    log!("no ui hooked");
    log!("app finished starting");
}

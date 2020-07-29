//! Bindings for ovr-toolkit custom apps js api.
//!
//! Based on [wiki/CustomApps](http://wiki.ovrtoolkit.co.uk/index.php?title=CustomApps&oldid=390)
//! and on [Zetaphor/ovrt-helper](https://github.com/Zetaphor/ovrt-helper/blob/524815728c19d431431f2f7a9a6e3ca7a6a19691/ovrt-helper.js).

#![allow(unused_unsafe)]

pub mod api;
pub mod consts;
pub mod events;
pub mod types;

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::{log_1, log_2};

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
    unsafe {
        log_2(&"spawn_overlay_callback:".into(), &uid.into());
    }
}

pub fn main() {
    unsafe { log_1(&"init main".into()) };
    let transform = types::OVROverlayTransform::default();

    unsafe { log_1(&"calling spawn_overlay".into()) };
    unsafe {
        api::spawn_overlay(&transform, "spawn_overlay_callback".into())
            .expect("failed to serialize the transform")
    };
    unsafe { log_1(&"called spawn_overlay".into()) };

    // window.SpawnOverlay(JSON.stringify(transform), "spawn_overlay_callback");

    // ovrt.createWebWin(
    //     `https://www.twitch.tv/popout/${username}/chat?popout=`,
    //     400,
    //     500
    //   );
}

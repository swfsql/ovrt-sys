//! Bindings for ovr-toolkit custom apps js api.
//!
//! Based on [wiki/CustomApps](http://wiki.ovrtoolkit.co.uk/index.php?title=CustomApps&oldid=390)
//! and on [Zetaphor/ovrt-helper](https://github.com/Zetaphor/ovrt-helper/blob/524815728c19d431431f2f7a9a6e3ca7a6a19691/ovrt-helper.js).

use wasm_bindgen::prelude::wasm_bindgen;

// This wrapper function is the primary modification we're making to the vanilla
// hello.rs example.
#[wasm_bindgen]
pub fn wasm_main() {
    // This hook is necessary to get panic messages in the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main()
}

pub fn main() {}

pub mod api;
pub mod consts;
pub mod events;
pub mod types;

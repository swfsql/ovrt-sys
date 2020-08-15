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
pub mod cmd;
pub mod consts;
pub mod events;
pub mod log;
pub mod types;

pub(crate) use log::js_value as v;
use wasm_bindgen::prelude::wasm_bindgen;
pub use web_sys;

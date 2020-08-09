/// Wrapper for the `log_<n>` functions.
#[macro_export]
macro_rules! log {
    () => {
        unsafe { web_sys::console::log_0() };
    };
    (  $t0:expr ) => {
        unsafe { web_sys::console::log_1(&$t0.into()) };
    };
    (  $t0:expr, $t1:expr ) => {
        unsafe { web_sys::console::log_2(&$t0.into(), &$t1.into()) };
    };
    (  $t0:expr, $t1:expr, $t2:expr ) => {
        unsafe { web_sys::console::log_3(&$t0.into(), &$t1.into(), &$t2.into()) };
    };
    (  $t0:expr, $t1:expr, $t2:expr, $t3:expr) => {
        unsafe { web_sys::console::log_4(&$t0.into(), &$t1.into(), &$t2.into(), &$t3.into()) };
    };
    (  $t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr) => {
        unsafe {
            web_sys::console::log_5(
                &$t0.into(),
                &$t1.into(),
                &$t2.into(),
                &$t3.into(),
                &$t4.into(),
            )
        };
    };
    (  $t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr, $t5:expr) => {
        unsafe {
            web_sys::console::log_6(
                &$t0.into(),
                &$t1.into(),
                &$t2.into(),
                &$t3.into(),
                &$t4.into(),
                &$t5.into(),
            )
        };
    };
    (  $t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr, $t5:expr, $t6:expr) => {
        unsafe {
            web_sys::console::log_7(
                &$t0.into(),
                &$t1.into(),
                &$t2.into(),
                &$t3.into(),
                &$t4.into(),
                &$t5.into(),
                &$t6.into(),
            )
        };
    };
}

/// `format!`-like log
#[allow(unused_macros)]
macro_rules! flog {
    ( $( $t:tt )* ) => {
        unsafe{web_sys::console::log_1(&format!( $( $t )* ).into())};
    }
}

pub fn js_value<T: serde::Serialize>(value: T) -> wasm_bindgen::JsValue {
    wasm_bindgen::JsValue::from_serde(&value).expect("failed to create a js_value")
}

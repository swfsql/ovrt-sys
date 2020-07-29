use crate::{api, consts};

/// Any kind of value, to be coupled with `Setting` into a `SettingValue`.
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum Value {
    F64(f64),
    I32(i32),
    Bool(bool),
}

impl From<&SettingValue> for Value {
    fn from(valued: &SettingValue) -> Value {
        use SettingValue::*;
        match valued {
            Size(v) => Value::F64(*v),
            Opacity(v) => Value::F64(*v),
            Curvature(v) => Value::F64(*v),
            Framerate(v) => Value::I32(*v),
            EcoMode(v) => Value::Bool(*v),
            LookHiding(v) => Value::Bool(*v),
            AttachedDevice(v) => Value::I32(*v),
        }
    }
}

/// This is Aa composition of `Setting` and `Value`,
/// used to change settings in an overlay.
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum SettingValue {
    /// (Width in meters).
    Size(f64),
    Opacity(f64),
    Curvature(f64),
    Framerate(i32),
    EcoMode(bool),
    LookHiding(bool),
    AttachedDevice(i32),
}

impl SettingValue {
    /// Given a type kind and a value, tries to compose a `SettingValue`.
    pub fn compose(kind: consts::Setting, value: Value) -> Option<Self> {
        kind.with(value)
    }
    /// Extracts the type kind and value.
    pub fn decompose(&self) -> (consts::Setting, Value) {
        (consts::Setting::from(self), Value::from(self))
    }
    /// Uses this type kind and value to set an overlay setting.
    ///
    /// TODO: if this presents a too high overhead, new abstractions
    /// can be introduced to call `set_overlay_setting_...` functions
    /// in a more direct manner.
    pub fn set_in_overlay(&self, uid: i32) {
        use SettingValue::*;
        let setting = consts::Setting::from(self) as i32;
        unsafe {
            match self {
                Size(v) => api::set_overlay_setting_f64(uid, setting, *v),
                Opacity(v) => api::set_overlay_setting_f64(uid, setting, *v),
                Curvature(v) => api::set_overlay_setting_f64(uid, setting, *v),
                Framerate(v) => api::set_overlay_setting_i32(uid, setting, *v),
                EcoMode(v) => api::set_overlay_setting_bool(uid, setting, *v),
                LookHiding(v) => api::set_overlay_setting_bool(uid, setting, *v),
                AttachedDevice(v) => api::set_overlay_setting_i32(uid, setting, *v),
            }
        }
    }
}

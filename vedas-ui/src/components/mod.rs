extern crate getrandom;
#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "card")]
pub mod card;
#[cfg(feature = "carousel")]
pub mod carousel;
#[cfg(feature = "dropdown")]
pub mod dropdown;
#[cfg(feature = "forms")]
pub mod forms;
#[cfg(feature = "layouts")]
pub mod layouts;
#[cfg(feature = "modal")]
pub mod modal;
#[cfg(feature = "navbar")]
pub mod navbar;
use std::borrow::Cow;
use std::fmt::Display;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{CustomEvent, Event};

pub mod slider;
#[cfg(feature = "spinner")]
pub mod spinner;
#[cfg(feature = "tabs")]
pub mod tabs;
#[cfg(feature = "text")]
pub mod text;
pub mod toggle;
pub mod tooltip;
pub mod toast;
pub mod accordion;
pub fn bool_to_option(value: bool) -> Option<Cow<'static, str>> {
    value.then(|| Cow::from("true"))
}

pub fn to_option_string(s: impl Display) -> Option<Cow<'static, str>> {
    let s = s.to_string();
    match s.as_str() {
        "" => None,
        _ => Some(Cow::from(s)),
    }
}

pub fn event_into_details(event: &Event) -> JsValue {
    JsValue::from(event)
        .dyn_into::<CustomEvent>()
        .unwrap_or_else(|_| panic!("could not convert to CustomEvent"))
        .detail()
}
pub fn event_details_into<T: JsCast>(event: &Event) -> T {
    event_into_details(event).unchecked_into::<T>()
}

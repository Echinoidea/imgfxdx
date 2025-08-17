use crate::components::Upload;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Upload {}
    }
}

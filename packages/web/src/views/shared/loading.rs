use dioxus::prelude::*;

#[component]
pub fn LoadingSpinner(#[props(default = "加载中...".to_string())] text: String) -> Element {
    rsx! {
        div {
            class: "loading-container",
            div { class: "spinner" }
            p { class: "loading-text", "{text}" }
        }
    }
}

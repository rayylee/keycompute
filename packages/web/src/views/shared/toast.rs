use dioxus::prelude::*;

use crate::stores::ui_store::{ToastKind, UiStore};

#[component]
pub fn Toast() -> Element {
    let ui_store = use_context::<UiStore>();
    let toast = ui_store.toast.read();

    if let Some(ref msg) = *toast {
        let kind_class = match msg.kind {
            ToastKind::Success => "toast-success",
            ToastKind::Error => "toast-error",
            ToastKind::Info => "toast-info",
            ToastKind::Warning => "toast-warning",
        };
        rsx! {
            div {
                class: "toast {kind_class}",
                p { class: "toast-title", "{msg.title}" }
                if let Some(ref text) = msg.message {
                    p { class: "toast-message", "{text}" }
                }
            }
        }
    } else {
        rsx! {}
    }
}

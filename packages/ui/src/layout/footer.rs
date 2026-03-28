use dioxus::prelude::*;

/// 页脚组件
#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            span { class: "footer-text",
                "© 2025 KeyCompute · Built with Dioxus 0.7"
            }
        }
    }
}

use dioxus::prelude::*;

#[component]
pub fn DistributionOverview() -> Element {
    rsx! {
        div {
            class: "page-container",
            div {
                class: "page-header",
                h1 { class: "page-title", "分发管理" }
                p { class: "page-subtitle", "管理您的 API Key 分发策略" }
            }
            div {
                class: "section",
                h2 { class: "section-title", "分发规则" }
                div { class: "empty-state", p { "暂无分发规则，敬请期待" } }
            }
        }
    }
}

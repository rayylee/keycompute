use dioxus::prelude::*;

#[component]
pub fn PaymentsOverview() -> Element {
    rsx! {
        div {
            class: "page-container",
            div {
                class: "page-header",
                h1 { class: "page-title", "支付与账单" }
            }
            div {
                class: "stats-grid",
                div {
                    class: "stat-card",
                    p { class: "stat-title", "账户余额" }
                    p { class: "stat-value", "¥ —" }
                }
                div {
                    class: "stat-card",
                    p { class: "stat-title", "本月消耗" }
                    p { class: "stat-value", "¥ —" }
                }
            }
            div {
                class: "section",
                div {
                    class: "section-header",
                    h2 { class: "section-title", "充值记录" }
                    a { class: "btn btn-primary", href: "/payments/recharge", "立即充值" }
                }
                div { class: "empty-state", p { "暂无充值记录" } }
            }
        }
    }
}

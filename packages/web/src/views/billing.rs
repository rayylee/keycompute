use dioxus::prelude::*;

use crate::router::Route;

/// 账单页面（已合并到支付与账单页面）
///
/// 该页面保留以维持 /billing 路由兼容，自动重定向到 /payments
#[component]
pub fn Billing() -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::PaymentsOverview {});
    });
    rsx! {
        div {
            class: "auth-redirect-loading",
            style: "display:flex;align-items:center;justify-content:center;height:100vh",
            span { style: "color:var(--text-secondary,#64748b)", "跳转中…" }
        }
    }
}

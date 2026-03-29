use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "error-page",
            div {
                class: "error-content",
                h1 { class: "error-code", "404" }
                h2 { class: "error-title", "页面不存在" }
                p { class: "error-desc", "您访问的页面不存在或已被移除" }
                a {
                    class: "btn btn-primary",
                    href: "/",
                    "返回首页"
                }
            }
        }
    }
}

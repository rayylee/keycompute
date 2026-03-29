use dioxus::prelude::*;

use crate::stores::user_store::UserStore;

#[component]
pub fn Dashboard() -> Element {
    let user_store = use_context::<UserStore>();
    let user_info = user_store.info.read();

    let greeting = if let Some(ref u) = *user_info {
        format!("你好，{}", u.name.as_deref().unwrap_or(&u.email))
    } else {
        "你好".to_string()
    };

    rsx! {
        div {
            class: "page-container",
            div {
                class: "page-header",
                h1 { class: "page-title", "{greeting}" }
                p { class: "page-subtitle", "这是您的控制台概览" }
            }

            div {
                class: "stats-grid",
                StatCard {
                    title: "API 调用次数",
                    value: "—",
                    label: "今日",
                    icon: "key",
                }
                StatCard {
                    title: "账户余额",
                    value: "—",
                    label: "可用",
                    icon: "wallet",
                }
                StatCard {
                    title: "活跃 Key",
                    value: "—",
                    label: "总计",
                    icon: "list",
                }
                StatCard {
                    title: "本月消耗",
                    value: "—",
                    label: "预估",
                    icon: "chart",
                }
            }

            div {
                class: "section",
                h2 { class: "section-title", "快速入口" }
                div {
                    class: "quick-links",
                    QuickLink { href: "/api-keys", label: "管理 API Key" }
                    QuickLink { href: "/payments", label: "充值余额" }
                    QuickLink { href: "/user/profile", label: "账户设置" }
                }
            }
        }
    }
}

#[component]
fn StatCard(title: String, value: String, label: String, icon: String) -> Element {
    rsx! {
        div {
            class: "stat-card",
            div {
                class: "stat-icon stat-icon-{icon}",
            }
            div {
                class: "stat-body",
                p { class: "stat-title", "{title}" }
                p { class: "stat-value", "{value}" }
                p { class: "stat-label", "{label}" }
            }
        }
    }
}

#[component]
fn QuickLink(href: String, label: String) -> Element {
    rsx! {
        a {
            class: "quick-link-card",
            href: "{href}",
            "{label}"
        }
    }
}

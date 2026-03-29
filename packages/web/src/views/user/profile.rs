use dioxus::prelude::*;

use crate::stores::user_store::UserStore;

#[component]
pub fn UserProfile() -> Element {
    let user_store = use_context::<UserStore>();
    let user_info = user_store.info.read();
    let display_name = user_info
        .as_ref()
        .map(|u| u.name.as_deref().unwrap_or("-").to_string())
        .unwrap_or_default();
    let email = user_info
        .as_ref()
        .map(|u| u.email.clone())
        .unwrap_or_default();
    let role = user_info
        .as_ref()
        .map(|u| u.role.clone())
        .unwrap_or_default();
    let avatar = user_info.as_ref().map(|u| u.avatar_char()).unwrap_or('U');
    let has_user = user_info.is_some();

    rsx! {
        div {
            class: "page-container",
            div {
                class: "page-header",
                h1 { class: "page-title", "个人资料" }
            }
            div {
                class: "card",
                if has_user {
                    div {
                        class: "profile-avatar",
                        span { class: "avatar-char", "{avatar}" }
                    }
                    div {
                        class: "profile-info",
                        div {
                            class: "form-group",
                            label { class: "form-label", "姓名" }
                            p { class: "form-value", "{display_name}" }
                        }
                        div {
                            class: "form-group",
                            label { class: "form-label", "邮箱" }
                            p { class: "form-value", "{email}" }
                        }
                        div {
                            class: "form-group",
                            label { class: "form-label", "角色" }
                            p { class: "form-value", "{role}" }
                        }
                    }
                } else {
                    div { class: "empty-state", p { "加载中..." } }
                }
            }
        }
    }
}

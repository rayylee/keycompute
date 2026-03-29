use dioxus::prelude::*;

#[component]
pub fn UserSettings() -> Element {
    rsx! {
        div {
            class: "page-container",
            div {
                class: "page-header",
                h1 { class: "page-title", "账户设置" }
            }
            div {
                class: "card",
                h2 { class: "section-title", "修改密码" }
                form {
                    div {
                        class: "form-group",
                        label { class: "form-label", "当前密码" }
                        input { class: "form-input", r#type: "password", placeholder: "请输入当前密码" }
                    }
                    div {
                        class: "form-group",
                        label { class: "form-label", "新密码" }
                        input { class: "form-input", r#type: "password", placeholder: "请输入新密码" }
                    }
                    div {
                        class: "form-group",
                        label { class: "form-label", "确认新密码" }
                        input { class: "form-input", r#type: "password", placeholder: "再次输入新密码" }
                    }
                    button {
                        class: "btn btn-primary",
                        r#type: "submit",
                        "保存修改"
                    }
                }
            }
        }
    }
}

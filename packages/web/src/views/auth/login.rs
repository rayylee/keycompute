use dioxus::prelude::*;

use crate::stores::auth_store::AuthStore;

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut auth_store = use_context::<AuthStore>();

    let on_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        let email_val = email();
        let password_val = password();
        if email_val.is_empty() || password_val.is_empty() {
            error_msg.set(Some("请填写邮箱和密码".to_string()));
            return;
        }
        loading.set(true);
        error_msg.set(None);
        spawn(async move {
            auth_store.login(email_val, password_val);
            loading.set(false);
        });
    };

    rsx! {
        div {
            class: "auth-page",
            div {
                class: "auth-card",
                div {
                    class: "auth-header",
                    h1 { class: "auth-title", "登录" }
                    p { class: "auth-subtitle", "登录您的账户以继续" }
                }

                if let Some(err) = error_msg() {
                    div {
                        class: "alert alert-error",
                        "{err}"
                    }
                }

                form {
                    onsubmit: on_submit,
                    div {
                        class: "form-group",
                        label { class: "form-label", "邮箱" }
                        input {
                            class: "form-input",
                            r#type: "email",
                            placeholder: "请输入邮箱",
                            value: "{email}",
                            oninput: move |e| email.set(e.value()),
                        }
                    }
                    div {
                        class: "form-group",
                        label { class: "form-label", "密码" }
                        input {
                            class: "form-input",
                            r#type: "password",
                            placeholder: "请输入密码",
                            value: "{password}",
                            oninput: move |e| password.set(e.value()),
                        }
                    }
                    div {
                        class: "form-actions",
                        a {
                            class: "link",
                            href: "/auth/forgot-password",
                            "忘记密码？"
                        }
                    }
                    button {
                        class: "btn btn-primary btn-full",
                        r#type: "submit",
                        disabled: loading(),
                        if loading() { "登录中..." } else { "登录" }
                    }
                }

                div {
                    class: "auth-footer",
                    "还没有账户？"
                    a {
                        class: "link",
                        href: "/auth/register",
                        " 立即注册"
                    }
                }
            }
        }
    }
}

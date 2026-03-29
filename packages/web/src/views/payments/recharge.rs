use dioxus::prelude::*;

#[component]
pub fn Recharge() -> Element {
    let mut amount = use_signal(String::new);
    let mut loading = use_signal(|| false);

    let on_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        if amount().is_empty() {
            return;
        }
        loading.set(true);
        spawn(async move {
            // TODO: call payment_service.create_order
            loading.set(false);
        });
    };

    rsx! {
        div {
            class: "page-container",
            div {
                class: "page-header",
                h1 { class: "page-title", "充值" }
            }
            div {
                class: "card",
                form {
                    onsubmit: on_submit,
                    div {
                        class: "form-group",
                        label { class: "form-label", "充值金额（元）" }
                        div {
                            class: "amount-presets",
                            for preset in ["10", "50", "100", "500"] {
                                button {
                                    class: "btn btn-outline",
                                    r#type: "button",
                                    onclick: move |_| amount.set(preset.to_string()),
                                    "¥{preset}"
                                }
                            }
                        }
                        input {
                            class: "form-input",
                            r#type: "number",
                            placeholder: "或输入自定义金额",
                            value: "{amount}",
                            oninput: move |e| amount.set(e.value()),
                        }
                    }
                    button {
                        class: "btn btn-primary btn-full",
                        r#type: "submit",
                        disabled: loading(),
                        if loading() { "处理中..." } else { "确认充值" }
                    }
                }
            }
        }
    }
}

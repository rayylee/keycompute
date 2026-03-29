use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct ApiKeyItem {
    id: String,
    name: String,
    prefix: String,
    status: String,
    created_at: String,
}

#[component]
pub fn ApiKeyList() -> Element {
    let mut show_create = use_signal(|| false);
    let mut new_key_name = use_signal(String::new);
    let mut loading = use_signal(|| false);

    let keys = use_signal(Vec::<ApiKeyItem>::new);

    let on_create = move |evt: Event<FormData>| {
        evt.prevent_default();
        if new_key_name().is_empty() {
            return;
        }
        loading.set(true);
        spawn(async move {
            // TODO: call api_key_service.create
            loading.set(false);
            show_create.set(false);
            new_key_name.set(String::new());
        });
    };

    rsx! {
        div {
            class: "page-container",
            div {
                class: "page-header",
                h1 { class: "page-title", "API Key 管理" }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| show_create.set(true),
                    "+ 创建 API Key"
                }
            }

            if show_create() {
                div {
                    class: "modal-overlay",
                    div {
                        class: "modal",
                        h2 { class: "modal-title", "创建 API Key" }
                        form {
                            onsubmit: on_create,
                            div {
                                class: "form-group",
                                label { class: "form-label", "名称" }
                                input {
                                    class: "form-input",
                                    r#type: "text",
                                    placeholder: "为此 Key 取个名字",
                                    value: "{new_key_name}",
                                    oninput: move |e| new_key_name.set(e.value()),
                                }
                            }
                            div {
                                class: "modal-actions",
                                button {
                                    class: "btn btn-ghost",
                                    r#type: "button",
                                    onclick: move |_| show_create.set(false),
                                    "取消"
                                }
                                button {
                                    class: "btn btn-primary",
                                    r#type: "submit",
                                    disabled: loading(),
                                    if loading() { "创建中..." } else { "创建" }
                                }
                            }
                        }
                    }
                }
            }

            div {
                class: "table-container",
                if keys.read().is_empty() {
                    div {
                        class: "empty-state",
                        p { "暂无 API Key，点击上方按钮创建" }
                    }
                } else {
                    table {
                        class: "table",
                        thead {
                            tr {
                                th { "名称" }
                                th { "前缀" }
                                th { "状态" }
                                th { "创建时间" }
                                th { "操作" }
                            }
                        }
                        tbody {
                            for key in keys.read().iter() {
                                tr {
                                    key: "{key.id}",
                                    td { "{key.name}" }
                                    td { code { "{key.prefix}..." } }
                                    td {
                                        span {
                                            class: "badge badge-{key.status}",
                                            "{key.status}"
                                        }
                                    }
                                    td { "{key.created_at}" }
                                    td {
                                        button {
                                            class: "btn btn-sm btn-danger",
                                            "删除"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

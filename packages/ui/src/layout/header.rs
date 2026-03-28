use dioxus::prelude::*;

use crate::icons::{IconBell, IconChevronDown, IconGlobe, IconMenu, IconMoon, IconSun};

/// 顶部栏组件
///
/// # Props
/// - `page_title`：当前页面标题
/// - `user_name`：当前用户名（头像首字母）
/// - `sidebar_collapsed`：侧边栏折叠状态（Signal，点击汉堡菜单时切换）
/// - `sidebar_mobile_open`：移动端侧边栏开关（Signal）
/// - `theme`：当前主题（Signal<String>），值为 "light" / "dark" / "system"
/// - `lang`：当前语言（Signal<String>），值为 "zh" / "en"
#[component]
pub fn Header(
    #[props(default)] page_title: String,
    #[props(default)] user_name: String,
    sidebar_collapsed: Signal<bool>,
    sidebar_mobile_open: Signal<bool>,
    theme: Signal<String>,
    lang: Signal<String>,
) -> Element {
    // 头像首字母
    let avatar_char = user_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "U".to_string());

    // 主题图标：light 显示月亮（切换到 dark），dark 显示太阳（切换到 light）
    let is_dark = theme() == "dark";
    let theme_title = if is_dark { "切换到亮色主题" } else { "切换到暗色主题" };

    let lang_val = lang();
    let lang_label = if lang_val == "zh" { "EN" } else { "中" };
    let lang_title = if lang_val == "zh" { "Switch to English" } else { "切换到中文" };

    let title = page_title.clone();

    rsx! {
        header { class: "header",
            // 左侧
            div { class: "header-left",
                // PC 端折叠/展开按钮
                button {
                    class: "header-toggle-btn hide-mobile",
                    title: "切换侧边栏",
                    onclick: move |_| {
                        let cur = sidebar_collapsed();
                        *sidebar_collapsed.write() = !cur;
                    },
                    IconMenu { size: 20 }
                }
                // 移动端汉堡菜单
                button {
                    class: "header-toggle-btn hide-desktop hide-tablet",
                    title: "打开菜单",
                    onclick: move |_| {
                        let cur = sidebar_mobile_open();
                        *sidebar_mobile_open.write() = !cur;
                    },
                    IconMenu { size: 20 }
                }

                // 页面标题
                if !title.is_empty() {
                    h1 { class: "header-page-title", "{title}" }
                }
            }

            // 右侧工具栏
            div { class: "header-right",
                // 主题切换
                button {
                    class: "header-icon-btn",
                    title: "{theme_title}",
                    onclick: move |_| {
                        let cur = theme();
                        let next = if cur == "dark" { "light" } else { "dark" };
                        *theme.write() = next.to_string();
                        // 持久化到 localStorage
                        #[cfg(target_arch = "wasm32")]
                        {
                            let _ = write_local_storage("theme", next);
                        }
                    },
                    if is_dark {
                        IconSun { size: 18 }
                    } else {
                        IconMoon { size: 18 }
                    }
                }

                // 语言切换
                button {
                    class: "header-icon-btn",
                    title: "{lang_title}",
                    style: "font-size: 12px; font-weight: 600; width: 36px;",
                    onclick: move |_| {
                        let cur = lang();
                        let next = if cur == "zh" { "en" } else { "zh" };
                        *lang.write() = next.to_string();
                        #[cfg(target_arch = "wasm32")]
                        {
                            let _ = write_local_storage("lang", next);
                        }
                    },
                    IconGlobe { size: 18 }
                    span { style: "font-size: 11px; margin-left: 2px;", "{lang_label}" }
                }

                // 通知
                button {
                    class: "header-icon-btn",
                    title: "通知",
                    IconBell { size: 18 }
                }

                // 用户头像
                div { class: "header-avatar",
                    title: "{user_name}",
                    "{avatar_char}"
                }

                // 用户名 + 下拉箭头（桌面端）
                button {
                    class: "header-icon-btn hide-mobile",
                    style: "gap: 4px; width: auto; padding: 0 4px;",
                    span {
                        style: "font-size: 13px; font-weight: 500; color: var(--text-primary); max-width: 120px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                        "{user_name}"
                    }
                    IconChevronDown { size: 16 }
                }
            }
        }
    }
}

// ── localStorage 写入 ────────────────────────────
#[cfg(target_arch = "wasm32")]
fn write_local_storage(key: &str, value: &str) -> Option<()> {
    use wasm_bindgen::JsCast;
    web_sys::window()?
        .local_storage()
        .ok()??
        .set_item(key, value)
        .ok()
}

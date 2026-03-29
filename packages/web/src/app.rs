use dioxus::prelude::*;

use crate::i18n::Lang;
use crate::router::Route;
use crate::stores::{auth_store::AuthStore, ui_store::UiStore, user_store::UserStore};
use crate::views::shared::Toast;
use ui::layout::sidebar::NavIcon;
use ui::{AppShell, NavItem, NavSection};

/// 根组件：提供所有全局 context，挂载路由
#[component]
pub fn App() -> Element {
    // 全局 context providers（必须在组件树顶层调用）
    let _auth_store = use_context_provider(AuthStore::new);
    let _user_store = use_context_provider(UserStore::new);
    let _ui_store = use_context_provider(UiStore::new);
    let _lang = use_context_provider(|| use_signal(Lang::default));

    rsx! {
        Router::<Route> {}
    }
}

/// 带 AppShell 侧边栏布局的页面外壳
#[component]
pub fn AppLayout() -> Element {
    let nav_sections = vec![
        NavSection {
            title: None,
            items: vec![
                NavItem::new("控制台", "/", NavIcon::Home),
                NavItem::new("API Key", "/api-keys", NavIcon::Key),
            ],
        },
        NavSection {
            title: Some("账务".to_string()),
            items: vec![
                NavItem::new("支付与账单", "/payments", NavIcon::Wallet),
                NavItem::new("分发管理", "/distribution", NavIcon::Share),
            ],
        },
        NavSection {
            title: Some("账户".to_string()),
            items: vec![
                NavItem::new("个人资料", "/user/profile", NavIcon::User),
                NavItem::new("账户设置", "/user/settings", NavIcon::Settings),
            ],
        },
    ];

    rsx! {
        AppShell {
            nav_sections,
            Toast {}
            Outlet::<Route> {}
        }
    }
}

//! KeyCompute 共享 UI 组件库
//!
//! # 模块结构
//! - `layout`  — 应用布局（AppShell、Sidebar、Header、Footer）
//! - `icons`   — 内联 SVG 图标组件

pub mod icons;
pub mod layout;

// Re-export 最常用的布局类型，方便外部直接 `use ui::AppShell`
pub use layout::app_shell::UiState;
pub use layout::{AppShell, Footer, Header, NavIcon, NavItem, NavSection, Sidebar};

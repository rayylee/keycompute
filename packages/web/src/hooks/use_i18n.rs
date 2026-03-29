use dioxus::prelude::*;

use crate::i18n::{I18n, Lang};

/// 获取国际化实例（从 Context 中读取语言 Signal）
pub fn use_i18n() -> I18n {
    let lang_signal = use_context::<Signal<Lang>>();
    I18n::new(lang_signal())
}

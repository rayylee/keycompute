#![allow(dead_code)]

use dioxus::prelude::*;

use crate::stores::user_store::UserStore;

/// 获取用户信息 Store（从 Context 中读取）
pub fn use_user() -> UserStore {
    use_context::<UserStore>()
}

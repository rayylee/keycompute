#![allow(dead_code)]

use dioxus::prelude::*;

use crate::stores::auth_store::AuthStore;

/// 获取认证 Store（从 Context 中读取）
pub fn use_auth() -> AuthStore {
    use_context::<AuthStore>()
}

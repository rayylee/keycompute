//! 用户管理相关类型

use serde::{Deserialize, Serialize};

use crate::api::common::encode_query_value;

/// 用户查询参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct UserQueryParams {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub role: Option<String>,
}

impl UserQueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();
        if let Some(limit) = self.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = self.offset {
            params.push(format!("offset={}", offset));
        }
        if let Some(ref role) = self.role {
            params.push(format!("role={}", encode_query_value(role)));
        }
        params.join("&")
    }
}

/// 用户详情
#[derive(Debug, Clone, Deserialize)]
pub struct UserDetail {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
    pub tenant_id: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 更新用户请求
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub role: Option<String>,
}

impl UpdateUserRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }
}

/// 更新余额请求
#[derive(Debug, Clone, Serialize)]
pub struct UpdateBalanceRequest {
    pub amount: f64,
    pub operation: String,
    pub reason: Option<String>,
}

impl UpdateBalanceRequest {
    pub fn add(amount: f64) -> Self {
        Self {
            amount,
            operation: "add".to_string(),
            reason: None,
        }
    }

    pub fn subtract(amount: f64) -> Self {
        Self {
            amount,
            operation: "subtract".to_string(),
            reason: None,
        }
    }

    pub fn set(amount: f64) -> Self {
        Self {
            amount,
            operation: "set".to_string(),
            reason: None,
        }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }
}

/// 余额响应
#[derive(Debug, Clone, Deserialize)]
pub struct BalanceResponse {
    pub user_id: String,
    pub balance: f64,
    pub currency: String,
}

/// API Key 信息（用于 Admin 查看用户 API Key 列表）
#[derive(Debug, Clone, Deserialize)]
pub struct ApiKeyInfo {
    pub id: String,
    pub name: String,
    pub key_preview: String,
    pub revoked: bool,
    pub created_at: String,
}

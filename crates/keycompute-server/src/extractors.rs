//! 提取器
//!
//! 自定义 Axum 提取器，用于从请求中提取认证信息等

use crate::error::{ApiError, Result};
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap},
};
use serde::{Deserialize, Serialize};
use std::future::Future;
use uuid::Uuid;

/// 认证提取器
///
/// 从请求头中提取 API Key 并解析用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthExtractor {
    /// 用户 ID
    pub user_id: Uuid,
    /// 租户 ID
    pub tenant_id: Uuid,
    /// API Key ID
    pub api_key_id: Uuid,
}

impl AuthExtractor {
    /// 创建新的认证提取器（用于测试）
    pub fn new(user_id: Uuid, tenant_id: Uuid, api_key_id: Uuid) -> Self {
        Self {
            user_id,
            tenant_id,
            api_key_id,
        }
    }

    /// 从 Authorization 头解析
    pub fn from_header(headers: &HeaderMap) -> Result<Self> {
        let auth_header = headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| ApiError::Auth("Missing Authorization header".to_string()))?;

        // 解析 Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| ApiError::Auth("Invalid Authorization format".to_string()))?;

        // TODO: 实际应该调用 auth 服务验证 token
        // 这里简化处理，仅作演示
        if token.starts_with("sk-") {
            // 模拟返回
            Ok(Self {
                user_id: Uuid::new_v4(),
                tenant_id: Uuid::new_v4(),
                api_key_id: Uuid::new_v4(),
            })
        } else {
            Err(ApiError::Auth("Invalid API key".to_string()))
        }
    }
}

impl<S> FromRequestParts<S> for AuthExtractor
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = std::result::Result<Self, Self::Rejection>> + Send {
        async move { Self::from_header(&parts.headers) }
    }
}

/// 请求 ID 提取器
#[derive(Debug, Clone)]
pub struct RequestId(pub Uuid);

impl RequestId {
    /// 创建新的请求 ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> FromRequestParts<S> for RequestId
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = std::result::Result<Self, Self::Rejection>> + Send {
        async move {
            // 尝试从 X-Request-ID 头获取，否则生成新的
            let id = parts
                .headers
                .get("X-Request-ID")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| Uuid::parse_str(s).ok())
                .unwrap_or_else(Uuid::new_v4);
            
            Ok(Self(id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn test_auth_extractor_from_header_valid() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_static("Bearer sk-test123"));

        let result = AuthExtractor::from_header(&headers);
        assert!(result.is_ok());
    }

    #[test]
    fn test_auth_extractor_from_header_missing() {
        let headers = HeaderMap::new();
        let result = AuthExtractor::from_header(&headers);
        assert!(matches!(result, Err(ApiError::Auth(_))));
    }

    #[test]
    fn test_auth_extractor_from_header_invalid_format() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_static("Basic dXNlcjpwYXNz"));

        let result = AuthExtractor::from_header(&headers);
        assert!(matches!(result, Err(ApiError::Auth(_))));
    }

    #[test]
    fn test_request_id_new() {
        let id = RequestId::new();
        assert_ne!(id.0, Uuid::nil());
    }
}

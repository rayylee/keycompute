//! API Key 验证
//!
//! 处理 API Key 的验证和解析。

use keycompute_types::{KeyComputeError, Result};
use keycompute_db::{ApiKey, User};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{AuthContext, Permission};

/// API Key 验证器
#[derive(Clone)]
pub struct ApiKeyValidator {
    /// 数据库连接池（可选）
    pool: Option<Arc<PgPool>>,
    /// 密钥（用于无数据库时的回退验证）
    secret: String,
}

impl std::fmt::Debug for ApiKeyValidator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiKeyValidator")
            .field("pool", &self.pool.as_ref().map(|_| "PgPool"))
            .field("secret", &"***")
            .finish()
    }
}

impl ApiKeyValidator {
    /// 创建新的 API Key 验证器（无数据库连接）
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            pool: None,
            secret: secret.into(),
        }
    }

    /// 创建带数据库连接的验证器
    pub fn with_pool(pool: Arc<PgPool>) -> Self {
        Self {
            pool: Some(pool),
            secret: String::new(),
        }
    }

    /// 验证 API Key
    ///
    /// API Key 格式: `sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx`
    pub async fn validate(&self, key: &str) -> Result<AuthContext> {
        // 检查格式
        if !key.starts_with("sk-") {
            return Err(KeyComputeError::AuthError(
                "Invalid API key format".into(),
            ));
        }

        // 计算 key 的 hash
        let key_hash = Self::hash_key(key);

        // 尝试从数据库验证
        if let Some(pool) = &self.pool {
            return self.validate_from_database(pool, &key_hash).await;
        }

        // 无数据库连接，使用回退逻辑
        self.validate_fallback(&key_hash).await
    }

    /// 从数据库验证 API Key
    async fn validate_from_database(
        &self,
        pool: &PgPool,
        key_hash: &str,
    ) -> Result<AuthContext> {
        // 查询 API Key
        let api_key = ApiKey::find_by_hash(pool, key_hash)
            .await
            .map_err(|e| KeyComputeError::DatabaseError(format!("Failed to query API key: {}", e)))?;

        let Some(api_key) = api_key else {
            tracing::warn!(key_hash = %key_hash, "API key not found");
            return Err(KeyComputeError::AuthError("Invalid API key".into()));
        };

        // 检查是否有效
        if !api_key.is_valid() {
            tracing::warn!(
                api_key_id = %api_key.id,
                revoked = api_key.revoked,
                "API key is not valid"
            );
            return Err(KeyComputeError::AuthError(
                "API key is revoked or expired".into(),
            ));
        }

        // 查询用户信息
        let user = User::find_by_id(pool, api_key.user_id)
            .await
            .map_err(|e| KeyComputeError::DatabaseError(format!("Failed to query user: {}", e)))?;

        let Some(user) = user else {
            tracing::warn!(user_id = %api_key.user_id, "User not found");
            return Err(KeyComputeError::AuthError("User not found".into()));
        };

        // 更新最后使用时间
        let _ = api_key.update_last_used(pool).await;

        tracing::info!(
            user_id = %user.id,
            tenant_id = %user.tenant_id,
            api_key_id = %api_key.id,
            "API key validated successfully"
        );

        // 构建权限列表
        let permissions = match user.role.as_str() {
            "admin" => vec![
                Permission::UseApi,
                Permission::ManageUsers,
                Permission::ManageApiKeys,
                Permission::ViewBilling,
                Permission::ManageBilling,
            ],
            "user" => vec![Permission::UseApi, Permission::ViewBilling],
            _ => vec![Permission::UseApi],
        };

        Ok(AuthContext {
            user_id: user.id,
            tenant_id: user.tenant_id,
            api_key_id: api_key.id,
            role: user.role,
            permissions,
        })
    }

    /// 回退验证（无数据库时使用）
    async fn validate_fallback(&self, key_hash: &str) -> Result<AuthContext> {
        tracing::debug!(key_hash = %key_hash, "Validating API key (fallback mode)");

        // 模拟验证成功
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let api_key_id = Uuid::new_v4();

        Ok(AuthContext {
            user_id,
            tenant_id,
            api_key_id,
            role: "user".to_string(),
            permissions: vec![Permission::UseApi],
        })
    }

    /// 计算 API Key 的 SHA256 hash
    pub fn hash_key(key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// 生成新的 API Key
    pub fn generate_key() -> String {
        let uuid = Uuid::new_v4();
        format!("sk-{}", uuid.to_string().replace("-", ""))
    }
}

impl Default for ApiKeyValidator {
    fn default() -> Self {
        Self::new("default-secret")
    }
}

/// API Key 认证 trait
#[async_trait::async_trait]
pub trait ApiKeyAuth: Send + Sync {
    /// 验证 API Key
    async fn authenticate(&self, key: &str) -> Result<AuthContext>;
}

#[async_trait::async_trait]
impl ApiKeyAuth for ApiKeyValidator {
    async fn authenticate(&self, key: &str) -> Result<AuthContext> {
        self.validate(key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key() {
        let key = ApiKeyValidator::generate_key();
        assert!(key.starts_with("sk-"));
        assert_eq!(key.len(), 35); // "sk-" + 32 个字符
    }

    #[test]
    fn test_hash_key() {
        let key = "sk-test123";
        let hash1 = ApiKeyValidator::hash_key(key);
        let hash2 = ApiKeyValidator::hash_key(key);
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA256 hex 长度
    }

    #[tokio::test]
    async fn test_validate_invalid_format() {
        let validator = ApiKeyValidator::new("secret");
        let result = validator.validate("invalid-key").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_valid_format() {
        let validator = ApiKeyValidator::new("secret");
        let key = ApiKeyValidator::generate_key();
        let result = validator.validate(&key).await;
        assert!(result.is_ok());

        let ctx = result.unwrap();
        assert!(!ctx.is_admin());
        assert!(ctx.has_permission(&Permission::UseApi));
    }
}

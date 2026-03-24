//! 认证配置

use serde::Deserialize;

/// 认证配置
#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    /// JWT 密钥（用于签名和验证）
    pub jwt_secret: String,
    /// JWT 签发者
    pub jwt_issuer: String,
    /// JWT 过期时间（秒）
    pub jwt_expiry_secs: u64,
    /// API Key 密钥（用于生成和验证 API Key）
    pub api_key_secret: String,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "change-me-in-production".to_string(),
            jwt_issuer: "keycompute".to_string(),
            jwt_expiry_secs: 3600,
            api_key_secret: "change-me-in-production".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_auth_config() {
        let config = AuthConfig::default();
        assert_eq!(config.jwt_issuer, "keycompute");
        assert_eq!(config.jwt_expiry_secs, 3600);
    }
}

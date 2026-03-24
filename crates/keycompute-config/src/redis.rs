//! Redis 配置

use serde::Deserialize;

/// Redis 配置
#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    /// Redis 连接 URL
    pub url: String,
    /// 键前缀（可选，用于多租户隔离）
    pub key_prefix: Option<String>,
    /// 连接池大小
    pub pool_size: Option<u32>,
    /// 连接超时（秒）
    pub connect_timeout_secs: Option<u64>,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1:6379".to_string(),
            key_prefix: None,
            pool_size: Some(10),
            connect_timeout_secs: Some(5),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_redis_config() {
        let config = RedisConfig::default();
        assert_eq!(config.url, "redis://127.0.0.1:6379");
        assert!(config.key_prefix.is_none());
        assert_eq!(config.pool_size, Some(10));
    }
}

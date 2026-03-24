//! 限流配置

use serde::Deserialize;

/// 限流配置
#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    /// 每分钟请求数限制
    pub rpm_limit: u32,
    /// 每分钟 Token 数限制
    pub tpm_limit: u32,
    /// 并发请求限制
    pub concurrency_limit: u32,
    /// 限流窗口大小（秒）
    pub window_secs: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            rpm_limit: 60,
            tpm_limit: 10000,
            concurrency_limit: 10,
            window_secs: 60,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ratelimit_config() {
        let config = RateLimitConfig::default();
        assert_eq!(config.rpm_limit, 60);
        assert_eq!(config.tpm_limit, 10000);
        assert_eq!(config.concurrency_limit, 10);
    }
}

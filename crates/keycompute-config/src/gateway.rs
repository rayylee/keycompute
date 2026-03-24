//! Gateway 配置

use serde::Deserialize;
use std::collections::HashMap;

/// Gateway 配置
#[derive(Debug, Deserialize, Clone)]
pub struct GatewayConfig {
    /// 最大重试次数
    pub max_retries: u32,
    /// 请求超时时间（秒）
    pub timeout_secs: u64,
    /// 是否启用 fallback
    pub enable_fallback: bool,
    /// HTTP 请求超时（秒）
    pub request_timeout_secs: u64,
    /// 流式请求超时（秒）
    pub stream_timeout_secs: u64,
    /// 重试策略配置
    pub retry: RetryConfig,
    /// HTTP 代理配置（可选）
    pub proxy: Option<ProxyConfig>,
}

/// 重试策略配置
#[derive(Debug, Deserialize, Clone)]
pub struct RetryConfig {
    /// 初始退避时间（毫秒）
    pub initial_backoff_ms: u64,
    /// 最大退避时间（毫秒）
    pub max_backoff_ms: u64,
    /// 退避倍数
    pub backoff_multiplier: f64,
}

/// HTTP 代理配置
#[derive(Debug, Deserialize, Clone)]
pub struct ProxyConfig {
    /// Provider 级代理映射
    /// 格式: {provider_name -> proxy_url}
    pub providers: HashMap<String, String>,
    /// 账号级代理映射（可选）
    /// 格式: {"provider:account_id" -> proxy_url}
    pub accounts: Option<HashMap<String, String>>,
    /// 通配符规则（可选）
    /// 格式: {pattern -> proxy_url}
    pub patterns: Option<HashMap<String, String>>,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            timeout_secs: 120,
            enable_fallback: true,
            request_timeout_secs: 120,
            stream_timeout_secs: 600,
            retry: RetryConfig::default(),
            proxy: None,
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            initial_backoff_ms: 100,
            max_backoff_ms: 10000,
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// 计算第 n 次重试的退避时间（毫秒）
    pub fn backoff_ms(&self, attempt: u32) -> u64 {
        if attempt == 0 {
            return 0;
        }

        let backoff = (self.initial_backoff_ms as f64
            * self.backoff_multiplier.powi((attempt - 1) as i32)) as u64;

        backoff.min(self.max_backoff_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_gateway_config() {
        let config = GatewayConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.timeout_secs, 120);
        assert!(config.enable_fallback);
        assert!(config.proxy.is_none());
    }

    #[test]
    fn test_retry_backoff() {
        let retry = RetryConfig::default();
        assert_eq!(retry.backoff_ms(0), 0);
        assert_eq!(retry.backoff_ms(1), 100);
        assert_eq!(retry.backoff_ms(2), 200);
        assert_eq!(retry.backoff_ms(3), 400);
    }

    #[test]
    fn test_proxy_config() {
        let mut providers = HashMap::new();
        providers.insert("openai".to_string(), "http://proxy-openai:8080".to_string());
        providers.insert("claude".to_string(), "http://proxy-claude:8080".to_string());

        let mut patterns = HashMap::new();
        patterns.insert("*-cn".to_string(), "http://cn-proxy:8080".to_string());

        let mut accounts = HashMap::new();
        accounts.insert(
            "openai:550e8400-e29b-41d4-a716-446655440000".to_string(),
            "http://premium-proxy:8080".to_string(),
        );

        let proxy = ProxyConfig {
            providers,
            patterns: Some(patterns),
            accounts: Some(accounts),
        };

        assert_eq!(proxy.providers.len(), 2);
        assert!(proxy.providers.contains_key("openai"));
        assert!(proxy.patterns.is_some());
        assert!(proxy.accounts.is_some());
    }
}

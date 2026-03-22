//! Gateway 调试接口
//!
//! 用于调试 Gateway 执行状态和 Provider 健康情况

use crate::{error::Result, state::AppState};
use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Gateway 状态响应
#[derive(Debug, Serialize)]
pub struct GatewayStatusResponse {
    /// Gateway 是否可用
    pub available: bool,
    /// 已加载的 Provider 列表
    pub providers: Vec<ProviderInfo>,
    /// 配置信息
    pub config: GatewayConfigInfo,
}

/// Provider 信息
#[derive(Debug, Serialize)]
pub struct ProviderInfo {
    /// Provider 名称
    pub name: String,
    /// 支持的模型列表
    pub supported_models: Vec<String>,
    /// 健康状态
    pub healthy: bool,
}

/// Gateway 配置信息
#[derive(Debug, Serialize)]
pub struct GatewayConfigInfo {
    /// 最大重试次数
    pub max_retries: u32,
    /// 超时时间（秒）
    pub timeout_secs: u64,
    /// 是否启用 fallback
    pub enable_fallback: bool,
}

impl Default for GatewayConfigInfo {
    fn default() -> Self {
        Self {
            max_retries: 3,
            timeout_secs: 120,
            enable_fallback: true,
        }
    }
}

/// 获取 Gateway 状态
pub async fn get_gateway_status(
    State(_state): State<AppState>,
) -> Result<Json<GatewayStatusResponse>> {
    // TODO: 从 GatewayExecutor 获取真实的 Provider 列表和状态
    // 目前返回模拟数据
    let providers = vec![
        ProviderInfo {
            name: "openai".to_string(),
            supported_models: vec![
                "gpt-4o".to_string(),
                "gpt-4o-mini".to_string(),
                "gpt-4-turbo".to_string(),
                "gpt-3.5-turbo".to_string(),
            ],
            healthy: true,
        },
    ];

    Ok(Json(GatewayStatusResponse {
        available: !providers.is_empty(),
        providers,
        config: GatewayConfigInfo::default(),
    }))
}

/// Provider 健康检查请求
#[derive(Debug, Deserialize)]
pub struct ProviderHealthRequest {
    /// Provider 名称
    pub provider: String,
    /// 测试用的 API Key（可选）
    pub api_key: Option<String>,
}

/// Provider 健康检查结果
#[derive(Debug, Serialize)]
pub struct ProviderHealthResponse {
    /// Provider 名称
    pub provider: String,
    /// 是否健康
    pub healthy: bool,
    /// 延迟（毫秒）
    pub latency_ms: Option<u64>,
    /// 错误信息（如果不健康）
    pub error: Option<String>,
    /// 支持的模型
    pub models: Vec<String>,
}

/// 检查 Provider 健康状态
pub async fn check_provider_health(
    State(_state): State<AppState>,
    Json(request): Json<ProviderHealthRequest>,
) -> Result<Json<ProviderHealthResponse>> {
    // TODO: 实现真实的健康检查逻辑
    // 目前返回模拟数据
    let healthy = request.provider == "openai";

    Ok(Json(ProviderHealthResponse {
        provider: request.provider,
        healthy,
        latency_ms: if healthy { Some(150) } else { None },
        error: if healthy { None } else { Some("Provider not configured".to_string()) },
        models: if healthy {
            vec![
                "gpt-4o".to_string(),
                "gpt-4o-mini".to_string(),
            ]
        } else {
            vec![]
        },
    }))
}

/// 执行统计信息
#[derive(Debug, Serialize)]
pub struct ExecutionStats {
    /// 总请求数
    pub total_requests: u64,
    /// 成功请求数
    pub successful_requests: u64,
    /// 失败请求数
    pub failed_requests: u64,
    /// Fallback 次数
    pub fallback_count: u64,
    /// 平均延迟（毫秒）
    pub avg_latency_ms: u64,
    /// Provider 统计
    pub provider_stats: HashMap<String, ProviderStats>,
}

/// Provider 统计
#[derive(Debug, Serialize)]
pub struct ProviderStats {
    /// 请求数
    pub requests: u64,
    /// 成功数
    pub successes: u64,
    /// 失败数
    pub failures: u64,
    /// 平均延迟
    pub avg_latency_ms: u64,
}

/// 获取执行统计
pub async fn get_execution_stats(
    State(_state): State<AppState>,
) -> Result<Json<ExecutionStats>> {
    // TODO: 从 GatewayExecutor 获取真实的统计数据
    // 目前返回模拟数据
    let mut provider_stats = HashMap::new();
    provider_stats.insert(
        "openai".to_string(),
        ProviderStats {
            requests: 100,
            successes: 95,
            failures: 5,
            avg_latency_ms: 150,
        },
    );

    Ok(Json(ExecutionStats {
        total_requests: 100,
        successful_requests: 95,
        failed_requests: 5,
        fallback_count: 3,
        avg_latency_ms: 150,
        provider_stats,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_config_info_default() {
        let config = GatewayConfigInfo::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.timeout_secs, 120);
        assert!(config.enable_fallback);
    }

    #[test]
    fn test_provider_health_request_deserialize() {
        let json = r#"{"provider": "openai", "api_key": "test-key"}"#;
        let req: ProviderHealthRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.provider, "openai");
        assert_eq!(req.api_key, Some("test-key".to_string()));
    }
}

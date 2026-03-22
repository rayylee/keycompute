//! 健康检查处理器

use axum::{
    http::StatusCode,
    response::Json,
};
use serde::Serialize;
use std::collections::HashMap;

/// 健康检查响应
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// 服务状态
    pub status: String,
    /// 版本
    pub version: String,
    /// 各组件状态
    pub components: HashMap<String, String>,
}

/// 健康检查
pub async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    let mut components = HashMap::new();
    components.insert("api".to_string(), "ok".to_string());
    components.insert("database".to_string(), "ok".to_string());

    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        components,
    };

    (StatusCode::OK, Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let (status, Json(response)) = health_check().await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(response.status, "healthy");
    }
}

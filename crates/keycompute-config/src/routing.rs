//! 路由配置

use serde::Deserialize;

/// 路由配置
#[derive(Debug, Deserialize, Clone)]
pub struct RoutingConfig {
    /// 成本权重
    pub cost_weight: f64,
    /// 延迟权重
    pub latency_weight: f64,
    /// 成功率权重
    pub success_weight: f64,
    /// 健康评分权重
    pub health_weight: f64,
    /// 不健康 Provider 的惩罚分数
    pub unhealthy_penalty: f64,
    /// 高延迟阈值（毫秒）
    pub high_latency_threshold_ms: u64,
}

impl Default for RoutingConfig {
    fn default() -> Self {
        Self {
            cost_weight: 0.3,
            latency_weight: 0.25,
            success_weight: 0.25,
            health_weight: 0.2,
            unhealthy_penalty: 100.0,
            high_latency_threshold_ms: 1000,
        }
    }
}

impl RoutingConfig {
    /// 获取权重总和
    pub fn total_weight(&self) -> f64 {
        self.cost_weight + self.latency_weight + self.success_weight + self.health_weight
    }

    /// 验证权重是否合理（总和接近 1.0）
    pub fn is_valid(&self) -> bool {
        let total = self.total_weight();
        (total - 1.0).abs() < 0.01
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_routing_config() {
        let config = RoutingConfig::default();
        assert_eq!(config.cost_weight, 0.3);
        assert_eq!(config.latency_weight, 0.25);
        assert!(config.is_valid());
    }

    #[test]
    fn test_total_weight() {
        let config = RoutingConfig::default();
        assert!((config.total_weight() - 1.0).abs() < 0.001);
    }
}

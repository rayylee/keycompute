//! 分销配置
//!
//! 提供二级分销的默认比例配置

use serde::{Deserialize, Serialize};

/// 分销配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionConfig {
    /// 一级分销默认比例（默认 3%，即 0.03）
    pub default_level1_ratio: f64,
    /// 二级分销默认比例（默认 2%，即 0.02）
    pub default_level2_ratio: f64,
    /// 最大总分销比例限制（默认 30%，即 0.30）
    pub max_total_ratio: f64,
}

impl Default for DistributionConfig {
    fn default() -> Self {
        Self {
            default_level1_ratio: 0.03, // 3%
            default_level2_ratio: 0.02, // 2%
            max_total_ratio: 0.30,      // 30%
        }
    }
}

impl DistributionConfig {
    /// 创建默认配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建带自定义比例的配置
    pub fn with_ratios(level1: f64, level2: f64) -> Self {
        Self {
            default_level1_ratio: level1,
            default_level2_ratio: level2,
            max_total_ratio: 0.30,
        }
    }

    /// 获取一级分销比例
    pub fn level1_ratio(&self) -> f64 {
        self.default_level1_ratio
    }

    /// 获取二级分销比例
    pub fn level2_ratio(&self) -> f64 {
        self.default_level2_ratio
    }

    /// 验证配置有效性
    pub fn validate(&self) -> Result<(), String> {
        if self.default_level1_ratio < 0.0 || self.default_level1_ratio > 1.0 {
            return Err("一级分销比例必须在 0-1 之间".to_string());
        }
        if self.default_level2_ratio < 0.0 || self.default_level2_ratio > 1.0 {
            return Err("二级分销比例必须在 0-1 之间".to_string());
        }
        if self.max_total_ratio < 0.0 || self.max_total_ratio > 1.0 {
            return Err("最大总分销比例必须在 0-1 之间".to_string());
        }
        if self.default_level1_ratio + self.default_level2_ratio > self.max_total_ratio {
            return Err("分销比例总和不能超过最大限制".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DistributionConfig::default();
        assert_eq!(config.default_level1_ratio, 0.03);
        assert_eq!(config.default_level2_ratio, 0.02);
        assert_eq!(config.max_total_ratio, 0.30);
    }

    #[test]
    fn test_custom_ratios() {
        let config = DistributionConfig::with_ratios(0.05, 0.03);
        assert_eq!(config.default_level1_ratio, 0.05);
        assert_eq!(config.default_level2_ratio, 0.03);
    }

    #[test]
    fn test_validate_valid() {
        let config = DistributionConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_ratio() {
        let config = DistributionConfig {
            default_level1_ratio: 1.5,
            default_level2_ratio: 0.02,
            max_total_ratio: 0.30,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_exceed_max() {
        let config = DistributionConfig {
            default_level1_ratio: 0.20,
            default_level2_ratio: 0.20,
            max_total_ratio: 0.30,
        };
        assert!(config.validate().is_err());
    }
}

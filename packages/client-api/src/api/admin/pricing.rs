//! 定价管理相关类型

use serde::{Deserialize, Serialize};

/// 定价信息
#[derive(Debug, Clone, Deserialize)]
pub struct PricingInfo {
    pub id: String,
    pub model: String,
    pub input_price: f64,
    pub output_price: f64,
    pub currency: String,
    pub is_default: bool,
    pub created_at: String,
}

/// 创建定价请求
#[derive(Debug, Clone, Serialize)]
pub struct CreatePricingRequest {
    pub model: String,
    pub input_price: f64,
    pub output_price: f64,
    pub currency: String,
}

impl CreatePricingRequest {
    pub fn new(
        model: impl Into<String>,
        input_price: f64,
        output_price: f64,
        currency: impl Into<String>,
    ) -> Self {
        Self {
            model: model.into(),
            input_price,
            output_price,
            currency: currency.into(),
        }
    }
}

/// 更新定价请求
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdatePricingRequest {
    pub input_price: Option<f64>,
    pub output_price: Option<f64>,
    pub currency: Option<String>,
}

impl UpdatePricingRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_input_price(mut self, price: f64) -> Self {
        self.input_price = Some(price);
        self
    }

    pub fn with_output_price(mut self, price: f64) -> Self {
        self.output_price = Some(price);
        self
    }
}

/// 设置默认定价请求
#[derive(Debug, Clone, Serialize)]
pub struct SetDefaultPricingRequest {
    pub model_ids: Vec<String>,
}

/// 计算费用请求
#[derive(Debug, Clone, Serialize)]
pub struct CalculateCostRequest {
    pub model: String,
    pub input_tokens: i64,
    pub output_tokens: i64,
}

/// 费用计算响应
#[derive(Debug, Clone, Deserialize)]
pub struct CostCalculationResponse {
    pub model: String,
    pub input_cost: f64,
    pub output_cost: f64,
    pub total_cost: f64,
    pub currency: String,
}

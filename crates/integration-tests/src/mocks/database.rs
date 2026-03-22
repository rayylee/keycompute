//! 模拟数据库

use rust_decimal::Decimal;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 模拟 UsageLog 记录
#[derive(Debug, Clone)]
pub struct MockUsageLog {
    pub id: Uuid,
    pub request_id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub api_key_id: Uuid,
    pub model_name: String,
    pub provider_name: String,
    pub account_id: Uuid,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub total_tokens: i32,
    pub input_unit_price_snapshot: Decimal,
    pub output_unit_price_snapshot: Decimal,
    pub user_amount: Decimal,
    pub currency: String,
    pub usage_source: String,
    pub status: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
}

impl MockUsageLog {
    pub fn new(ctx: &super::MockExecutionContext) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            request_id: ctx.request_id,
            tenant_id: ctx.tenant_id,
            user_id: ctx.user_id,
            api_key_id: ctx.api_key_id,
            model_name: ctx.model.clone(),
            provider_name: ctx.provider.clone(),
            account_id: ctx.account_id,
            input_tokens: 10,
            output_tokens: 4,
            total_tokens: 14,
            input_unit_price_snapshot: Decimal::from(1),
            output_unit_price_snapshot: Decimal::from(2),
            user_amount: Decimal::from(18) / Decimal::from(1000), // (10*1 + 4*2) / 1000
            currency: "CNY".to_string(),
            usage_source: "gateway_accumulated".to_string(),
            status: "success".to_string(),
            started_at: now,
            finished_at: now,
        }
    }

    pub fn with_tokens(mut self, input: i32, output: i32) -> Self {
        self.input_tokens = input;
        self.output_tokens = output;
        self.total_tokens = input + output;
        self.user_amount = self.calculate_amount();
        self
    }

    pub fn with_pricing(
        mut self,
        input_price: Decimal,
        output_price: Decimal,
    ) -> Self {
        self.input_unit_price_snapshot = input_price;
        self.output_unit_price_snapshot = output_price;
        self.user_amount = self.calculate_amount();
        self
    }

    fn calculate_amount(&self) -> Decimal {
        let input_cost = Decimal::from(self.input_tokens) * self.input_unit_price_snapshot
            / Decimal::from(1000);
        let output_cost = Decimal::from(self.output_tokens) * self.output_unit_price_snapshot
            / Decimal::from(1000);
        input_cost + output_cost
    }
}

/// 模拟 DistributionRecord
#[derive(Debug, Clone)]
pub struct MockDistributionRecord {
    pub id: Uuid,
    pub usage_log_id: Uuid,
    pub tenant_id: Uuid,
    pub beneficiary_id: Uuid,
    pub share_ratio: Decimal,
    pub share_amount: Decimal,
    pub currency: String,
    pub status: String,
}

impl MockDistributionRecord {
    pub fn new(usage_log: &MockUsageLog, beneficiary_id: Uuid, ratio: Decimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            usage_log_id: usage_log.id,
            tenant_id: usage_log.tenant_id,
            beneficiary_id,
            share_ratio: ratio,
            share_amount: usage_log.user_amount * ratio,
            currency: usage_log.currency.clone(),
            status: "pending".to_string(),
        }
    }
}

/// 内存数据库模拟
#[derive(Debug, Default)]
pub struct MockDatabase {
    usage_logs: std::sync::Mutex<Vec<MockUsageLog>>,
    distribution_records: std::sync::Mutex<Vec<MockDistributionRecord>>,
}

impl MockDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_usage_log(&self, log: MockUsageLog) {
        self.usage_logs.lock().unwrap().push(log);
    }

    pub fn insert_distribution_record(&self, record: MockDistributionRecord) {
        self.distribution_records.lock().unwrap().push(record);
    }

    pub fn get_usage_logs(&self) -> Vec<MockUsageLog> {
        self.usage_logs.lock().unwrap().clone()
    }

    pub fn get_distribution_records(&self) -> Vec<MockDistributionRecord> {
        self.distribution_records.lock().unwrap().clone()
    }

    pub fn get_usage_log_by_request(&self, request_id: Uuid) -> Option<MockUsageLog> {
        self.usage_logs
            .lock()
            .unwrap()
            .iter()
            .find(|log| log.request_id == request_id)
            .cloned()
    }

    pub fn clear(&self) {
        self.usage_logs.lock().unwrap().clear();
        self.distribution_records.lock().unwrap().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_usage_log() {
        let ctx = super::super::MockExecutionContext::new();
        let log = MockUsageLog::new(&ctx);
        
        assert_eq!(log.request_id, ctx.request_id);
        assert_eq!(log.model_name, ctx.model);
        assert!(log.user_amount > Decimal::ZERO);
    }

    #[test]
    fn test_mock_database() {
        let db = MockDatabase::new();
        let ctx = super::super::MockExecutionContext::new();
        let log = MockUsageLog::new(&ctx);
        
        db.insert_usage_log(log.clone());
        
        let logs = db.get_usage_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].request_id, ctx.request_id);
    }
}

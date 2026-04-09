use crate::DbError;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres, Transaction};
use uuid::Uuid;

/// 分销记录模型
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DistributionRecord {
    pub id: Uuid,
    pub usage_log_id: Uuid,
    pub tenant_id: Uuid,
    pub beneficiary_id: Uuid,
    pub share_amount: BigDecimal,
    pub share_ratio: BigDecimal,
    /// 分销层级: level1, level2
    pub level: String,
    pub status: String,
    pub settled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// 创建分销记录请求
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDistributionRecordRequest {
    pub usage_log_id: Uuid,
    pub tenant_id: Uuid,
    pub beneficiary_id: Uuid,
    pub share_amount: BigDecimal,
    pub share_ratio: BigDecimal,
    /// 分销层级: level1, level2
    pub level: String,
}

/// 分销统计
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct DistributionStats {
    pub total_records: i64,
    pub total_amount: BigDecimal,
    pub settled_amount: BigDecimal,
    pub pending_amount: BigDecimal,
}

/// 分销层级统计
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct DistributionLevelStats {
    /// 一级分销收益
    pub level1_amount: BigDecimal,
    /// 二级分销收益
    pub level2_amount: BigDecimal,
    /// 一级分销记录数
    pub level1_count: i64,
    /// 二级分销记录数
    pub level2_count: i64,
}

impl DistributionRecord {
    /// 创建分销记录
    pub async fn create(
        pool: &sqlx::PgPool,
        req: &CreateDistributionRecordRequest,
    ) -> Result<DistributionRecord, DbError> {
        let record = sqlx::query_as::<_, DistributionRecord>(
            r#"
            INSERT INTO distribution_records (
                usage_log_id, tenant_id, beneficiary_id,
                share_amount, share_ratio, level, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, 'pending')
            RETURNING *
            "#,
        )
        .bind(req.usage_log_id)
        .bind(req.tenant_id)
        .bind(req.beneficiary_id)
        .bind(&req.share_amount)
        .bind(&req.share_ratio)
        .bind(&req.level)
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    /// 批量创建分销记录（使用事务）
    ///
    /// 所有记录在同一事务中创建，保证原子性。
    /// 如果记录已存在（基于唯一约束），则跳过插入。
    pub async fn create_many(
        pool: &sqlx::PgPool,
        requests: &[CreateDistributionRecordRequest],
    ) -> Result<Vec<DistributionRecord>, DbError> {
        let mut tx = pool.begin().await?;
        let records = Self::create_many_tx(&mut tx, requests).await?;
        tx.commit().await?;
        Ok(records)
    }

    /// 批量创建分销记录（在现有事务中执行）
    ///
    /// 用于在调用者已有事务中执行批量插入。
    /// 使用 ON CONFLICT DO NOTHING 实现幂等性，已存在的记录将被跳过。
    pub async fn create_many_tx(
        tx: &mut Transaction<'_, Postgres>,
        requests: &[CreateDistributionRecordRequest],
    ) -> Result<Vec<DistributionRecord>, DbError> {
        let mut records = Vec::with_capacity(requests.len());

        for req in requests {
            // 使用 ON CONFLICT DO NOTHING 实现幂等性
            // 如果记录已存在（基于 uk_distribution_records_unique 约束），则跳过
            let record_result = sqlx::query_as::<_, DistributionRecord>(
                r#"
                INSERT INTO distribution_records (
                    usage_log_id, tenant_id, beneficiary_id,
                    share_amount, share_ratio, level, status
                )
                VALUES ($1, $2, $3, $4, $5, $6, 'pending')
                ON CONFLICT (usage_log_id, beneficiary_id, level) DO NOTHING
                RETURNING *
                "#,
            )
            .bind(req.usage_log_id)
            .bind(req.tenant_id)
            .bind(req.beneficiary_id)
            .bind(&req.share_amount)
            .bind(&req.share_ratio)
            .bind(&req.level)
            .fetch_optional(&mut **tx)
            .await?;

            // 如果记录已存在，查询现有记录
            if let Some(record) = record_result {
                records.push(record);
            } else {
                tracing::debug!(
                    usage_log_id = %req.usage_log_id,
                    beneficiary_id = %req.beneficiary_id,
                    level = %req.level,
                    "Distribution record already exists, skipping"
                );
                // 查询已存在的记录（使用 fetch_optional 避免并发删除导致错误）
                match sqlx::query_as::<_, DistributionRecord>(
                    r#"
                    SELECT * FROM distribution_records
                    WHERE usage_log_id = $1 AND beneficiary_id = $2 AND level = $3
                    "#,
                )
                .bind(req.usage_log_id)
                .bind(req.beneficiary_id)
                .bind(&req.level)
                .fetch_optional(&mut **tx)
                .await
                {
                    Ok(Some(existing)) => records.push(existing),
                    Ok(None) => {
                        // 记录可能已被并发删除，记录警告但不中断流程
                        tracing::warn!(
                            usage_log_id = %req.usage_log_id,
                            beneficiary_id = %req.beneficiary_id,
                            level = %req.level,
                            "Distribution record not found after conflict, possibly deleted concurrently"
                        );
                    }
                    Err(e) => {
                        tracing::error!(
                            usage_log_id = %req.usage_log_id,
                            beneficiary_id = %req.beneficiary_id,
                            level = %req.level,
                            error = %e,
                            "Failed to fetch existing distribution record"
                        );
                        return Err(e.into());
                    }
                }
            }
        }

        Ok(records)
    }

    /// 根据 ID 查找分销记录
    pub async fn find_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
    ) -> Result<Option<DistributionRecord>, DbError> {
        let record = sqlx::query_as::<_, DistributionRecord>(
            "SELECT * FROM distribution_records WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

    /// 查找用量日志的所有分销记录
    pub async fn find_by_usage_log(
        pool: &sqlx::PgPool,
        usage_log_id: Uuid,
    ) -> Result<Vec<DistributionRecord>, DbError> {
        let records = sqlx::query_as::<_, DistributionRecord>(
            "SELECT * FROM distribution_records WHERE usage_log_id = $1",
        )
        .bind(usage_log_id)
        .fetch_all(pool)
        .await?;

        Ok(records)
    }

    /// 查找租户的分销记录
    pub async fn find_by_tenant(
        pool: &sqlx::PgPool,
        tenant_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DistributionRecord>, DbError> {
        let records = sqlx::query_as::<_, DistributionRecord>(
            r#"
            SELECT * FROM distribution_records
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(tenant_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(records)
    }

    /// 查找受益人的分销记录
    pub async fn find_by_beneficiary(
        pool: &sqlx::PgPool,
        beneficiary_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DistributionRecord>, DbError> {
        let records = sqlx::query_as::<_, DistributionRecord>(
            r#"
            SELECT * FROM distribution_records
            WHERE beneficiary_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(beneficiary_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(records)
    }

    /// 结算分销记录
    pub async fn settle(&self, pool: &sqlx::PgPool) -> Result<DistributionRecord, DbError> {
        let record = sqlx::query_as::<_, DistributionRecord>(
            r#"
            UPDATE distribution_records
            SET status = 'settled',
                settled_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(self.id)
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    /// 获取受益人统计
    pub async fn get_stats_by_beneficiary(
        pool: &sqlx::PgPool,
        beneficiary_id: Uuid,
    ) -> Result<DistributionStats, DbError> {
        let stats = sqlx::query_as::<_, DistributionStats>(
            r#"
            SELECT
                COUNT(*) as total_records,
                COALESCE(SUM(share_amount), 0) as total_amount,
                COALESCE(SUM(CASE WHEN status = 'settled' THEN share_amount ELSE 0 END), 0) as settled_amount,
                COALESCE(SUM(CASE WHEN status = 'pending' THEN share_amount ELSE 0 END), 0) as pending_amount
            FROM distribution_records
            WHERE beneficiary_id = $1
            "#,
        )
        .bind(beneficiary_id)
        .fetch_one(pool)
        .await?;

        Ok(stats)
    }

    /// 获取受益人按层级的统计
    pub async fn get_level_stats_by_beneficiary(
        pool: &sqlx::PgPool,
        beneficiary_id: Uuid,
    ) -> Result<DistributionLevelStats, DbError> {
        let stats = sqlx::query_as::<_, DistributionLevelStats>(
            r#"
            SELECT
                COALESCE(SUM(CASE WHEN level = 'level1' THEN share_amount ELSE 0 END), 0) as level1_amount,
                COALESCE(SUM(CASE WHEN level = 'level2' THEN share_amount ELSE 0 END), 0) as level2_amount,
                COUNT(CASE WHEN level = 'level1' THEN 1 END) as level1_count,
                COUNT(CASE WHEN level = 'level2' THEN 1 END) as level2_count
            FROM distribution_records
            WHERE beneficiary_id = $1
            "#,
        )
        .bind(beneficiary_id)
        .fetch_one(pool)
        .await?;

        Ok(stats)
    }

    /// 获取受益人在某个 usage_log 下的总收益（用于推荐人收益显示）
    pub async fn get_earnings_for_referral(
        pool: &sqlx::PgPool,
        beneficiary_id: Uuid,
        referred_user_id: Uuid,
    ) -> Result<BigDecimal, DbError> {
        // 查询该推荐用户产生的所有分销收益
        let result: Option<(BigDecimal,)> = sqlx::query_as(
            r#"
            SELECT COALESCE(SUM(dr.share_amount), 0)
            FROM distribution_records dr
            JOIN usage_logs ul ON dr.usage_log_id = ul.id
            WHERE dr.beneficiary_id = $1 AND ul.user_id = $2
            "#,
        )
        .bind(beneficiary_id)
        .bind(referred_user_id)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|r| r.0).unwrap_or(BigDecimal::from(0)))
    }
}

-- 为 distribution_records 添加 level 字段
-- 用于明确标识分销层级（level1 或 level2）

ALTER TABLE distribution_records
ADD COLUMN level VARCHAR(20) NOT NULL DEFAULT 'level1';

-- 创建索引以支持按层级查询
CREATE INDEX idx_distribution_records_level ON distribution_records(level);

-- 更新已有数据（根据 share_ratio 推断层级）
-- share_ratio > 2% 的为 level1，否则为 level2
UPDATE distribution_records
SET level = CASE
    WHEN share_ratio > 0.02 THEN 'level1'
    ELSE 'level2'
END
WHERE level = 'level1';

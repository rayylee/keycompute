-- user_referrals: 用户推荐关系表
-- 用于存储谁推荐了谁，支持二级分销

CREATE TABLE user_referrals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -- 被推荐人（新用户）
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    -- 一级推荐人
    level1_referrer_id UUID REFERENCES users(id) ON DELETE SET NULL,
    -- 二级推荐人（推荐人的推荐人）
    level2_referrer_id UUID REFERENCES users(id) ON DELETE SET NULL,
    -- 推荐来源（可选，如推荐码、链接等）
    source VARCHAR(255),
    -- 推荐状态: pending, active, expired
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 索引
CREATE INDEX idx_user_referrals_user ON user_referrals(user_id);
CREATE INDEX idx_user_referrals_level1 ON user_referrals(level1_referrer_id);
CREATE INDEX idx_user_referrals_level2 ON user_referrals(level2_referrer_id);
CREATE INDEX idx_user_referrals_status ON user_referrals(status);

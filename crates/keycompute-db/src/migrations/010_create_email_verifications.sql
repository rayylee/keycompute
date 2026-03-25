-- email_verifications: 邮箱验证令牌表
-- 管理用户邮箱验证流程

CREATE TABLE email_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    -- 验证令牌 (随机字符串)
    token VARCHAR(255) NOT NULL UNIQUE,
    -- 令牌过期时间
    expires_at TIMESTAMPTZ NOT NULL,
    -- 是否已使用
    used BOOLEAN NOT NULL DEFAULT FALSE,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- 每个用户的每个邮箱只能有一个有效验证记录
    UNIQUE(user_id, email)
);

-- 索引
CREATE INDEX idx_email_verifications_token ON email_verifications(token);
CREATE INDEX idx_email_verifications_expires ON email_verifications(expires_at) 
    WHERE used = FALSE;
CREATE INDEX idx_email_verifications_user ON email_verifications(user_id);

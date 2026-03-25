-- user_credentials: 用户密码凭证表
-- 存储用户密码哈希和登录安全相关信息

CREATE TABLE user_credentials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    -- 密码哈希 (argon2id)
    password_hash VARCHAR(255) NOT NULL,
    -- 邮箱验证状态
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    email_verified_at TIMESTAMPTZ,
    -- 登录失败计数（用于防护暴力破解）
    failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    locked_until TIMESTAMPTZ,
    -- 最后登录信息
    last_login_at TIMESTAMPTZ,
    last_login_ip INET,
    -- 时间戳
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- 唯一约束：一个用户只有一个凭证记录
    UNIQUE(user_id)
);

-- 索引
CREATE INDEX idx_user_credentials_user ON user_credentials(user_id);
CREATE INDEX idx_user_credentials_locked ON user_credentials(locked_until) 
    WHERE locked_until IS NOT NULL;
CREATE INDEX idx_user_credentials_verified ON user_credentials(email_verified) 
    WHERE email_verified = FALSE;

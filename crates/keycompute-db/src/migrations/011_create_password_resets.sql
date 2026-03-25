-- password_resets: 密码重置令牌表
-- 管理用户密码重置流程

CREATE TABLE password_resets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    -- 重置令牌
    token VARCHAR(255) NOT NULL UNIQUE,
    -- 令牌过期时间（短时效，如 1 小时）
    expires_at TIMESTAMPTZ NOT NULL,
    -- 是否已使用
    used BOOLEAN NOT NULL DEFAULT FALSE,
    used_at TIMESTAMPTZ,
    -- 请求来源 IP
    requested_from_ip INET,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 索引
CREATE INDEX idx_password_resets_token ON password_resets(token);
CREATE INDEX idx_password_resets_expires ON password_resets(expires_at) 
    WHERE used = FALSE;
CREATE INDEX idx_password_resets_user ON password_resets(user_id);

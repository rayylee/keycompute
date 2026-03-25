//! 密码哈希模块
//!
//! 使用 Argon2id 算法进行密码哈希和验证

use argon2::{
    Algorithm, Argon2, Params,
    password_hash::{
        PasswordHash as ArgonPasswordHash, PasswordHasher as ArgonPasswordHasher, PasswordVerifier,
        SaltString, rand_core::OsRng,
    },
};
use keycompute_types::{KeyComputeError, Result};

/// 密码哈希器
///
/// 使用 Argon2id 算法，提供内存困难型哈希，防止 GPU 破解
#[derive(Debug, Clone)]
pub struct PasswordHasher {
    /// Argon2 实例
    argon2: Argon2<'static>,
}

impl Default for PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordHasher {
    /// 创建新的密码哈希器
    ///
    /// 使用默认的安全参数：
    /// - 内存: 64 MB
    /// - 迭代次数: 3
    /// - 并行度: 4
    /// - 输出长度: 32 bytes
    pub fn new() -> Self {
        let params = Params::new(
            65536,    // 64 MB memory
            3,        // 3 iterations
            4,        // 4 parallelism
            Some(32), // 32 bytes output
        )
        .expect("Failed to create Argon2 params");

        let argon2 = Argon2::new(Algorithm::Argon2id, argon2::Version::V0x13, params);

        Self { argon2 }
    }

    /// 使用自定义参数创建哈希器
    ///
    /// # Arguments
    /// * `memory_cost` - 内存成本（KB）
    /// * `time_cost` - 时间成本（迭代次数）
    /// * `parallelism` - 并行度
    pub fn with_params(memory_cost: u32, time_cost: u32, parallelism: u32) -> Result<Self> {
        let params = Params::new(memory_cost, time_cost, parallelism, Some(32)).map_err(|e| {
            KeyComputeError::AuthError(format!("Failed to create Argon2 params: {}", e))
        })?;

        let argon2 = Argon2::new(Algorithm::Argon2id, argon2::Version::V0x13, params);

        Ok(Self { argon2 })
    }

    /// 哈希密码
    ///
    /// 生成随机盐值并使用 Argon2id 哈希密码
    ///
    /// # Arguments
    /// * `password` - 明文密码
    ///
    /// # Returns
    /// 返回 PHC 格式的哈希字符串
    pub fn hash(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);

        let hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| KeyComputeError::AuthError(format!("Failed to hash password: {}", e)))?
            .to_string();

        Ok(hash)
    }

    /// 验证密码
    ///
    /// 验证明文密码是否与存储的哈希匹配
    ///
    /// # Arguments
    /// * `password` - 明文密码
    /// * `hash` - 存储的 PHC 格式哈希
    ///
    /// # Returns
    /// 返回 true 表示密码匹配
    pub fn verify(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = ArgonPasswordHash::new(hash).map_err(|e| {
            KeyComputeError::AuthError(format!("Invalid password hash format: {}", e))
        })?;

        match self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
        {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(KeyComputeError::AuthError(format!(
                "Failed to verify password: {}",
                e
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let hasher = PasswordHasher::new();
        let password = "SecurePassword123!";

        let hash = hasher.hash(password).expect("Failed to hash password");
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2id$"));

        assert!(hasher.verify(password, &hash).expect("Failed to verify"));
        assert!(
            !hasher
                .verify("WrongPassword", &hash)
                .expect("Failed to verify")
        );
    }

    #[test]
    fn test_unique_salts() {
        let hasher = PasswordHasher::new();
        let password = "SamePassword123!";

        let hash1 = hasher.hash(password).expect("Failed to hash");
        let hash2 = hasher.hash(password).expect("Failed to hash");

        // 相同密码应该产生不同的哈希（因为盐值不同）
        assert_ne!(hash1, hash2);

        // 但两个哈希都应该能验证原密码
        assert!(hasher.verify(password, &hash1).expect("Failed to verify"));
        assert!(hasher.verify(password, &hash2).expect("Failed to verify"));
    }

    #[test]
    fn test_custom_params() {
        let hasher = PasswordHasher::with_params(32768, 2, 2).expect("Failed to create hasher");
        let password = "TestPassword456@";

        let hash = hasher.hash(password).expect("Failed to hash");
        assert!(hasher.verify(password, &hash).expect("Failed to verify"));
    }
}

//! 验证器模块
//!
//! 提供密码强度验证和邮箱格式验证

use keycompute_types::{KeyComputeError, Result};
use regex::Regex;
use std::sync::OnceLock;

/// 密码验证器
///
/// 验证密码强度，确保密码满足最低安全要求
#[derive(Debug, Clone)]
pub struct PasswordValidator {
    /// 最小长度
    min_length: usize,
    /// 是否需要大写字母
    require_uppercase: bool,
    /// 是否需要小写字母
    require_lowercase: bool,
    /// 是否需要数字
    require_digit: bool,
    /// 是否需要特殊字符
    require_special: bool,
}

impl Default for PasswordValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordValidator {
    /// 创建新的密码验证器
    ///
    /// 默认要求：
    /// - 最小长度: 8 位
    /// - 需要大写字母
    /// - 需要小写字母
    /// - 需要数字
    /// - 需要特殊字符
    pub fn new() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_digit: true,
            require_special: true,
        }
    }

    /// 创建宽松的验证器（仅检查长度）
    pub fn lenient() -> Self {
        Self {
            min_length: 8,
            require_uppercase: false,
            require_lowercase: false,
            require_digit: false,
            require_special: false,
        }
    }

    /// 设置最小长度
    pub fn with_min_length(mut self, length: usize) -> Self {
        self.min_length = length;
        self
    }

    /// 设置是否需要大写字母
    pub fn with_uppercase(mut self, require: bool) -> Self {
        self.require_uppercase = require;
        self
    }

    /// 设置是否需要小写字母
    pub fn with_lowercase(mut self, require: bool) -> Self {
        self.require_lowercase = require;
        self
    }

    /// 设置是否需要数字
    pub fn with_digit(mut self, require: bool) -> Self {
        self.require_digit = require;
        self
    }

    /// 设置是否需要特殊字符
    pub fn with_special(mut self, require: bool) -> Self {
        self.require_special = require;
        self
    }

    /// 验证密码强度
    ///
    /// # Arguments
    /// * `password` - 待验证的密码
    ///
    /// # Returns
    /// 验证通过返回 Ok(())，失败返回错误信息
    pub fn validate(&self, password: &str) -> Result<()> {
        let mut errors = Vec::new();

        // 检查长度
        if password.len() < self.min_length {
            errors.push(format!("密码长度至少需要 {} 位", self.min_length));
        }

        // 检查大写字母
        if self.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            errors.push("密码需要包含至少一个大写字母".to_string());
        }

        // 检查小写字母
        if self.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            errors.push("密码需要包含至少一个小写字母".to_string());
        }

        // 检查数字
        if self.require_digit && !password.chars().any(|c| c.is_numeric()) {
            errors.push("密码需要包含至少一个数字".to_string());
        }

        // 检查特殊字符
        if self.require_special
            && !password
                .chars()
                .any(|c| "!@#$%^&*()_+-=[]{}|;:',.<>?/~`".contains(c))
        {
            errors.push("密码需要包含至少一个特殊字符".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(KeyComputeError::ValidationError(errors.join("; ")))
        }
    }

    /// 快速检查密码是否有效
    pub fn is_valid(&self, password: &str) -> bool {
        self.validate(password).is_ok()
    }
}

/// 邮箱验证器
///
/// 验证邮箱格式
#[derive(Debug, Clone)]
pub struct EmailValidator {
    /// 邮箱正则表达式
    pattern: Regex,
}

impl Default for EmailValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl EmailValidator {
    /// 创建新的邮箱验证器
    pub fn new() -> Self {
        static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
        let pattern = EMAIL_REGEX
            .get_or_init(|| {
                // 简化版邮箱正则，覆盖大部分常见场景
                Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
                    .expect("Invalid email regex")
            })
            .clone();

        Self { pattern }
    }

    /// 验证邮箱格式
    ///
    /// # Arguments
    /// * `email` - 待验证的邮箱
    ///
    /// # Returns
    /// 验证通过返回 Ok(())，失败返回错误信息
    pub fn validate(&self, email: &str) -> Result<()> {
        if email.is_empty() {
            return Err(KeyComputeError::ValidationError("邮箱不能为空".to_string()));
        }

        if email.len() > 255 {
            return Err(KeyComputeError::ValidationError(
                "邮箱长度不能超过255个字符".to_string(),
            ));
        }

        if !self.pattern.is_match(email) {
            return Err(KeyComputeError::ValidationError("邮箱格式无效".to_string()));
        }

        Ok(())
    }

    /// 快速检查邮箱是否有效
    pub fn is_valid(&self, email: &str) -> bool {
        self.validate(email).is_ok()
    }

    /// 规范化邮箱（转小写，去除首尾空格）
    pub fn normalize(&self, email: &str) -> String {
        email.trim().to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== PasswordValidator Tests ====================

    #[test]
    fn test_password_validator_valid() {
        let validator = PasswordValidator::new();

        assert!(validator.validate("SecurePass123!").is_ok());
        assert!(validator.validate("MyP@ssw0rd").is_ok());
        assert!(validator.validate("Abcdefg1!").is_ok());
    }

    #[test]
    fn test_password_validator_too_short() {
        let validator = PasswordValidator::new();

        assert!(validator.validate("Short1!").is_err());
        assert!(validator.validate("A1!b").is_err());
    }

    #[test]
    fn test_password_validator_no_uppercase() {
        let validator = PasswordValidator::new();

        assert!(validator.validate("securepass123!").is_err());
    }

    #[test]
    fn test_password_validator_no_lowercase() {
        let validator = PasswordValidator::new();

        assert!(validator.validate("SECUREPASS123!").is_err());
    }

    #[test]
    fn test_password_validator_no_digit() {
        let validator = PasswordValidator::new();

        assert!(validator.validate("SecurePass!!").is_err());
    }

    #[test]
    fn test_password_validator_no_special() {
        let validator = PasswordValidator::new();

        assert!(validator.validate("SecurePass123").is_err());
    }

    #[test]
    fn test_password_validator_lenient() {
        let validator = PasswordValidator::lenient();

        // 只要长度足够就通过
        assert!(validator.validate("simplepassword").is_ok());
        assert!(validator.validate("short").is_err());
    }

    #[test]
    fn test_password_validator_custom() {
        let validator = PasswordValidator::new()
            .with_min_length(12)
            .with_special(false);

        assert!(validator.validate("LongPassword123").is_ok());
        assert!(validator.validate("Short1!").is_err()); // 太短
    }

    // ==================== EmailValidator Tests ====================

    #[test]
    fn test_email_validator_valid() {
        let validator = EmailValidator::new();

        assert!(validator.validate("user@example.com").is_ok());
        assert!(validator.validate("test.email@domain.org").is_ok());
        assert!(validator.validate("user+tag@example.co.uk").is_ok());
        assert!(validator.validate("user123@test-domain.com").is_ok());
    }

    #[test]
    fn test_email_validator_invalid() {
        let validator = EmailValidator::new();

        assert!(validator.validate("").is_err());
        assert!(validator.validate("invalid").is_err());
        assert!(validator.validate("user@").is_err());
        assert!(validator.validate("@example.com").is_err());
        assert!(validator.validate("user@example").is_err());
        assert!(validator.validate("user @example.com").is_err());
    }

    #[test]
    fn test_email_validator_normalize() {
        let validator = EmailValidator::new();

        assert_eq!(
            validator.normalize("  User@Example.COM  "),
            "user@example.com"
        );
        assert_eq!(validator.normalize("TEST@DOMAIN.ORG"), "test@domain.org");
    }

    #[test]
    fn test_email_validator_too_long() {
        let validator = EmailValidator::new();

        let long_email = format!("{}@example.com", "a".repeat(250));
        assert!(validator.validate(&long_email).is_err());
    }
}

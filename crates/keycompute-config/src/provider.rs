//! Provider 配置

use serde::Deserialize;
use std::collections::HashMap;

/// Provider 配置
#[derive(Debug, Deserialize, Clone)]
pub struct ProviderConfig {
    /// OpenAI 配置
    pub openai: ProviderEndpoint,
    /// DeepSeek 配置
    pub deepseek: ProviderEndpoint,
    /// vLLM 配置（可选）
    pub vllm: Option<ProviderEndpoint>,
    /// Claude 配置（可选）
    pub claude: Option<ProviderEndpoint>,
    /// Gemini 配置（可选）
    pub gemini: Option<ProviderEndpoint>,
    /// Ollama 配置（可选）
    pub ollama: Option<ProviderEndpoint>,
    /// 自定义 Provider 配置
    pub custom: Option<HashMap<String, ProviderEndpoint>>,
}

/// Provider 端点配置
#[derive(Debug, Deserialize, Clone)]
pub struct ProviderEndpoint {
    /// API 端点 URL
    pub endpoint: String,
    /// API Key（可选，可以从环境变量读取）
    pub api_key: Option<String>,
    /// 请求超时（秒，可选）
    pub timeout_secs: Option<u64>,
    /// 支持的模型列表（可选）
    pub models: Option<Vec<String>>,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            openai: ProviderEndpoint {
                endpoint: "https://api.openai.com/v1".to_string(),
                api_key: None,
                timeout_secs: None,
                models: None,
            },
            deepseek: ProviderEndpoint {
                endpoint: "https://api.deepseek.com/v1".to_string(),
                api_key: None,
                timeout_secs: None,
                models: None,
            },
            vllm: None,
            claude: None,
            gemini: None,
            ollama: None,
            custom: None,
        }
    }
}

impl ProviderConfig {
    /// 获取指定 Provider 的配置
    pub fn get(&self, name: &str) -> Option<&ProviderEndpoint> {
        match name {
            "openai" => Some(&self.openai),
            "deepseek" => Some(&self.deepseek),
            "vllm" => self.vllm.as_ref(),
            "claude" => self.claude.as_ref(),
            "gemini" => self.gemini.as_ref(),
            "ollama" => self.ollama.as_ref(),
            _ => self.custom.as_ref().and_then(|c| c.get(name)),
        }
    }

    /// 获取所有已配置的 Provider 名称
    pub fn provider_names(&self) -> Vec<String> {
        let mut names = vec!["openai".to_string(), "deepseek".to_string()];

        if self.vllm.is_some() {
            names.push("vllm".to_string());
        }
        if self.claude.is_some() {
            names.push("claude".to_string());
        }
        if self.gemini.is_some() {
            names.push("gemini".to_string());
        }
        if self.ollama.is_some() {
            names.push("ollama".to_string());
        }

        if let Some(custom) = &self.custom {
            names.extend(custom.keys().cloned());
        }

        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_provider_config() {
        let config = ProviderConfig::default();
        assert_eq!(config.openai.endpoint, "https://api.openai.com/v1");
        assert_eq!(config.deepseek.endpoint, "https://api.deepseek.com/v1");
        assert!(config.vllm.is_none());
    }

    #[test]
    fn test_provider_names() {
        let config = ProviderConfig::default();
        let names = config.provider_names();
        assert!(names.contains(&"openai".to_string()));
        assert!(names.contains(&"deepseek".to_string()));
    }
}

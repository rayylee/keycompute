//! 模型列表处理器

use axum::Json;
use serde::{Deserialize, Serialize};

/// 模型信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    /// 模型 ID
    pub id: String,
    /// 对象类型
    pub object: String,
    /// 创建时间戳
    pub created: i64,
    /// 拥有者
    pub owned_by: String,
}

/// 模型列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelsResponse {
    /// 对象类型
    pub object: String,
    /// 模型列表
    pub data: Vec<ModelInfo>,
}

/// 获取模型列表
pub async fn list_models() -> Json<ModelsResponse> {
    let models = vec![
        ModelInfo {
            id: "gpt-4o".to_string(),
            object: "model".to_string(),
            created: 1715367049,
            owned_by: "openai".to_string(),
        },
        ModelInfo {
            id: "gpt-4o-mini".to_string(),
            object: "model".to_string(),
            created: 1721172741,
            owned_by: "openai".to_string(),
        },
        ModelInfo {
            id: "gpt-4-turbo".to_string(),
            object: "model".to_string(),
            created: 1712361441,
            owned_by: "openai".to_string(),
        },
        ModelInfo {
            id: "gpt-3.5-turbo".to_string(),
            object: "model".to_string(),
            created: 1677649963,
            owned_by: "openai".to_string(),
        },
    ];

    Json(ModelsResponse {
        object: "list".to_string(),
        data: models,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_models() {
        let Json(response) = list_models().await;
        assert_eq!(response.object, "list");
        assert!(!response.data.is_empty());
        assert_eq!(response.data[0].object, "model");
    }
}

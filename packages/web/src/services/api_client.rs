use client_api::{ApiClient, ClientConfig};

/// 获取 API 客户端实例
/// 从环境/编译配置中读取 base URL，开发时默认指向 localhost:8080
pub fn get_client() -> ApiClient {
    let base_url = option_env!("API_BASE_URL")
        .unwrap_or("http://localhost:8080")
        .to_string();

    let config = ClientConfig::new(base_url);
    ApiClient::new(config).expect("Failed to create API client")
}

use client_api::error::Result;
use client_api::{
    DebugApi,
    api::debug::{
        GatewayStats, GatewayStatus, HealthCheckResponse, ProviderHealthResponse, RoutingDebugInfo,
    },
};

use super::api_client::get_client;

pub async fn routing(token: &str) -> Result<RoutingDebugInfo> {
    let client = get_client();
    DebugApi::new(&client).debug_routing(token).await
}

pub async fn provider_health(token: &str) -> Result<ProviderHealthResponse> {
    let client = get_client();
    DebugApi::new(&client).get_provider_health(token).await
}

pub async fn gateway_status(token: &str) -> Result<GatewayStatus> {
    let client = get_client();
    DebugApi::new(&client).get_gateway_status(token).await
}

pub async fn gateway_stats(token: &str) -> Result<GatewayStats> {
    let client = get_client();
    DebugApi::new(&client).get_gateway_stats(token).await
}

pub async fn check_health(token: &str) -> Result<HealthCheckResponse> {
    let client = get_client();
    DebugApi::new(&client).check_provider_health(token).await
}

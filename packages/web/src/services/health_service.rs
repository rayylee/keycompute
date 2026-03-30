#![allow(dead_code)]

use client_api::error::Result;
use client_api::{HealthApi, api::health::HealthResponse};

use super::api_client::get_client;

pub async fn check() -> Result<HealthResponse> {
    let client = get_client();
    HealthApi::new(&client).health_check().await
}

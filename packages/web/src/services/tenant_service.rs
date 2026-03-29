use client_api::error::Result;
use client_api::{
    TenantApi,
    api::tenant::{TenantInfo, TenantQueryParams},
};

use super::api_client::get_client;

pub async fn list(params: Option<TenantQueryParams>, token: &str) -> Result<Vec<TenantInfo>> {
    let client = get_client();
    TenantApi::new(&client)
        .list_tenants(params.as_ref(), token)
        .await
}

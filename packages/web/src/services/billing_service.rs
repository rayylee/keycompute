use client_api::error::Result;
use client_api::{
    BillingApi,
    api::billing::{BillingQueryParams, BillingRecord, BillingStats},
};

use super::api_client::get_client;

pub async fn list(params: Option<BillingQueryParams>, token: &str) -> Result<Vec<BillingRecord>> {
    let client = get_client();
    BillingApi::new(&client)
        .list_billing_records(params.as_ref(), token)
        .await
}

pub async fn stats(token: &str) -> Result<BillingStats> {
    let client = get_client();
    BillingApi::new(&client).get_billing_stats(token).await
}

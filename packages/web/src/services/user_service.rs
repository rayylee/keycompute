use client_api::error::Result;
use client_api::{
    UserApi,
    api::user::{
        ChangePasswordRequest, CurrentUserResponse, MessageResponse, UpdateProfileRequest,
    },
};

use super::api_client::get_client;

pub async fn get_current_user(token: &str) -> Result<CurrentUserResponse> {
    let client = get_client();
    UserApi::new(&client).get_current_user(token).await
}

pub async fn update_profile(name: Option<String>, token: &str) -> Result<CurrentUserResponse> {
    let client = get_client();
    let req = UpdateProfileRequest { name };
    UserApi::new(&client).update_profile(&req, token).await
}

pub async fn change_password(current: &str, new: &str, token: &str) -> Result<MessageResponse> {
    let client = get_client();
    let req = ChangePasswordRequest::new(current, new);
    UserApi::new(&client).change_password(&req, token).await
}

use dioxus::prelude::*;

use crate::app::AppLayout;
use crate::views::{
    NotFound,
    api_keys::ApiKeyList,
    auth::{ForgotPassword, Login, Register},
    dashboard::Dashboard,
    distribution::DistributionOverview,
    payments::{PaymentsOverview, Recharge},
    user::{UserProfile, UserSettings},
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // 认证页面（无 AppShell 布局）
    #[route("/auth/login")]
    Login {},
    #[route("/auth/register")]
    Register {},
    #[route("/auth/forgot-password")]
    ForgotPassword {},

    // 主应用（带 AppShell 布局）
    #[layout(AppLayout)]
        #[route("/")]
        Dashboard {},
        #[route("/api-keys")]
        ApiKeyList {},
        #[route("/payments")]
        PaymentsOverview {},
        #[route("/payments/recharge")]
        Recharge {},
        #[route("/distribution")]
        DistributionOverview {},
        #[route("/user/profile")]
        UserProfile {},
        #[route("/user/settings")]
        UserSettings {},
    #[end_layout]

    // 404
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

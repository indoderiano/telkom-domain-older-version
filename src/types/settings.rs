use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChangePassword {
    enabled: bool,
    html: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GuardianMfaPage {
    enabled: bool,
    html: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrorPage {
    html: String,
    show_log_link: bool,
    url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeviceFlow {
    charset: String,
    mask: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Flags {
    change_pwd_flow_v1: bool,
    enable_apis_section: bool,
    disable_impersonation: bool,
    enable_client_connections: bool,
    enable_pipeline2: bool,
    allow_legacy_delegation_grant_types: bool,
    allow_legacy_ro_grant_types: bool,
    allow_legacy_tokeninfo_endpoint: bool,
    enable_legacy_profile: bool,
    enable_idtoken_api2: bool,
    enable_public_signup_user_exists_error: bool,
    enable_sso: bool,
    allow_changing_enable_sso: bool,
    disable_clickjack_protection_headers: bool,
    no_disclose_enterprise_connections: bool,
    enforce_client_authentication_on_passwordless_start: bool,
    enable_adfs_waad_email_verification: bool,
    revoke_refresh_token_grant: bool,
    dashboard_log_streams_next: bool,
    dashboard_insights_view: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SessionCookie {
    mode: String
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TenantSettings {
    change_password: ChangePassword,
    guardian_mfa_page: GuardianMfaPage,
    default_audience: String,
    default_directory: String,
    error_page: ErrorPage,
    device_flow: DeviceFlow,
    flags: Flags,
    friendly_name: String,
    picture_url: String,
    support_email: String,
    support_url: String,
    allowed_logout_urls: Vec<String>,
    session_lifetime: u64,
    idle_session_lifetime: u64,
    sandbox_version: String,
    sandbox_versions_available: Vec<String>,
    default_redirection_uri: String,
    enabled_locales: Vec<String>,
    session_cookie: SessionCookie,
}

// impl TenantSettings {
//     fn new() -> TenantSettings {
//         TenantSettings {
//             change_password: ChangePassword {

//             },
//             guardian_mfa_page: GuardianMfaPage,
//             default_audience: String,
//             default_directory: String,
//             error_page: ErrorPage,
//             device_flow: DeviceFlow,
//             flags: Flags,
//             friendly_name: String,
//             picture_url: String,
//             support_email: String,
//             support_url: String,
//             allowed_logout_urls: Vec<String>,
//             session_lifetime: u64,
//             idle_session_lifetime: u64,
//             sandbox_version: String,
//             sandbox_versions_available: Vec<String>,
//             default_redirection_uri: String,
//             enabled_locales: Vec<String>,
//             session_cookie: SessionCookie,
//         }
//     }
// }
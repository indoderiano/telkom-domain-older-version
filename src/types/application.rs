use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Debug, Clone)]
pub struct AppList {
    pub name: String,
    pub client_id: String,
    pub app_type: String,
    pub logo_uri: String,
}

// #[derive(Deserialize, Debug, Clone)]
// pub callbacks: String,
// pub allowed_origins: String,
// pub web_origins: String,
// pub client_aliases: String,
// pub allowed_clients: String,
// pub allowed_logout_urls: String,
// pub grant_types: String,

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseAppList {
    pub message: String,
    pub data: Vec<AppList>
}

// #[derive(Deserialize, Debug, Clone)]
// pub struct ResponseAppList {
//     pub message: String,
//     pub data: Vec<AppList>
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct JwtConfiguration {
//     pub lifetime_in_seconds: i32,
//     pub scopes: None,
//     pub alg: String,
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct EncryptionKey {
//     pub r#pub: String,
//     pub cert: String,
//     pub subject: String,
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct Addons {
//     pub aws: None,
//     pub azure_blob: None,
//     pub azure_sb: None,
//     pub rms: None,
//     pub mscrm: None,
//     pub slack: None,
//     pub sentry: None,
//     pub r#box: None,
//     pub cloudbees: None,
//     pub concur: None,
//     pub dropbox: None,
//     pub echosign: None,
//     pub egnyte: None,
//     pub firebase: None,
//     pub newrelic: None,
//     pub office365: None,
//     pub salesforce: None,
//     pub salesforce_api: None,
//     pub salesforce_sandbox_api: None,
//     pub samlp: None,
//     pub layer: None,
//     pub sap_api: None,
//     pub sharepoint: None,
//     pub springcm: None,
//     pub wams: None,
//     pub wsfed: None,
//     pub zendesk: None,
//     pub wsfed: None,
//     pub zendesk: None,
//     pub zoom: None,
//     pub sso_integration: None,
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct EncryptionKey {
//     pub r#pub: String,
//     pub cert: String,
//     pub subject: String,
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct Android {
//     pub app_package_name: String,
//     pub sha256_cert_fingerprints: Option<Vec<String>>,
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct Ios {
//     pub team_id: String,
//     pub app_bundle_identifier: String,
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct Mobile {
//     pub android: Android,
//     pub ios: Ios,
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct NativeSocialLogin {
//     pub apple: String,
//     pub facebook: String,
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct RefreshToken {
//     pub rotation_type: String,
//     pub expiration_type: String,
//     pub leeway: i32,
//     pub token_lifetime: i32,
//     pub infinite_token_lifetime: bool,
//     pub idle_token_lifetime: i32,
//     pub infinite_idle_token_lifetime: bool,

// }

// #[derive(Serialize, Debug, Clone)]
// pub struct AppCreate {
//     pub name: String,
//     pub identifier: String,
//     pub sign_algorithm: String,
//     pub name: String,
//     pub description: String,
//     pub logo_uri: String,
//     pub callbacks: Option<Vec<String>>,
//     pub allowed_origins: Option<Vec<String>>,
//     pub web_origins: Option<Vec<String>>,
//     pub client_aliases: Option<Vec<String>>,
//     pub allowed_clients: Option<Vec<String>>,
//     pub allowed_logout_urls: Option<Vec<String>>,
//     pub grant_types: Option<Vec<String>>,
//     pub token_endpoint_auth_method: String,
//     pub app_type: String,
//     pub is_first_party: bool,
//     pub oidc_conformant: bool,
//     pub jwt_configuration: JwtConfiguration,
//     pub sso: bool,
//     pub cross_origin_auth: bool,
//     pub cross_origin_loc: String,
//     pub sso_disabled: bool,
//     pub custom_login_page_on: bool,
//     pub custom_login_page: String,
//     pub custom_login_page_preview: String,
//     pub form_template: String,
//     pub form_template: Addons,
//     pub client_metadata: None,
//     pub mobile: Mobile,
//     pub initiate_login_uri: String,
//     pub native_social_login: NativeSocialLogin,
//     pub refresh_token: RefreshToken,
//     pub organization_usage: String,
//     pub organization_require_behavior: String,
// }

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct AppDetails {
    pub tenant: String,
    pub name: String,
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
    pub description: String,
    pub app_logo: String,
    pub app_type: String,
    pub authentication_method: String,
    pub login_url: String,
    pub allowed_urls : String,
    pub allowed_logout_urls : String,
    pub allowed_web_origins: String,
    pub allowed_origins: String,
    pub token_exp: u32,
    pub refresh_token_rotation: bool,
    pub refresh_token_rotation_interval: i32,
    pub refesh_token_absolute_expiration: bool,
    pub refesh_token_absolute_expiration_lifetime: i32,
    pub refesh_token_inactivity_expiration: bool,
    pub refesh_token_inactivity_expiration_lifetime: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseAppDetails {
    pub message: String,
    pub data: AppDetails
}

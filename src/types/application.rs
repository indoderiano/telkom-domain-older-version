use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Debug, Clone)]
pub struct AppList {
    pub name: String,
    pub client_id: String,
    pub app_type: String,
    // pub logo_uri: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct RefreshToken {
    pub expiration_type: String,
    pub leeway: i32,
    pub infinite_token_lifetime: bool,
    pub infinite_idle_token_lifetime: bool,
    pub token_lifetime: i32,
    pub idle_token_lifetime: i32,
    pub rotation_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct SigningKeys {
    pub cert: String,
    pub pkcs7: String,
    pub subject: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct JwtConfiguration {
    pub alg: String,
    pub lifetime_in_seconds: i32,
    pub secret_encoded: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct AppDetails {
    pub tenant: String,
    pub global: bool,
    pub is_token_endpoint_ip_header_trusted: bool,
    pub name: String,
    pub is_first_party: bool,
    pub oidc_conformant: bool,
    pub sso_disabled: bool,
    pub cross_origin_auth: bool,
    pub refresh_token: RefreshToken,
    pub encrypted: bool,
    pub allowed_clients: String,
    pub callbacks: String,
    pub signing_keys: SigningKeys,
    pub client_id: String,
    pub callback_url_template: bool,
    pub client_secret: String,
    pub jwt_configuration: JwtConfiguration,
    pub client_aliases: String,
    pub token_endpoint_auth_method: String,
    pub app_type: String,
    pub grant_types: String,
    pub custom_login_page_on: bool,
}

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Debug, Clone)]
pub struct AppType {
    pub id: i32,
    pub name: String,
    pub client_id: String,
    pub api_type: String,
    pub logo_url: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseAppList {
    pub message: String,
    pub data: Vec<AppType>
}

#[derive(Serialize, Debug, Clone)]
pub struct AppCreate {
    pub name: String,
    pub identifier: String,
    pub sign_algorithm: String,
}

// #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
// pub struct ApiDetails {
//     pub id: u8,
//     pub name: String,
//     pub api_id: String,
//     pub api_type: String,
//     pub identifier: String,
//     pub token_exp: u32,
//     pub token_exp_browser: u32,
//     pub sign_algorithm: String,
//     pub rbac: bool,
//     pub permission_acc_token: bool,
//     pub allow_skip_user: bool,
//     pub allow_off_acc: bool,
//     pub tenant_id: String,
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct ResponseApiDetails {
//     pub message: String,
//     pub data: ApiDetails
// }
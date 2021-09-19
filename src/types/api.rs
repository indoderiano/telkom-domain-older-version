use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Debug, Clone)]
pub struct ApiTitle {
    pub id: String,
    pub name: String,
    pub api_type: String,
    pub identifier: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiDetails {
    pub id: u8,
    pub name: String,
    pub api_type: String,
    pub identifier: String,
    pub token_exp: u32,
    pub token_exp_browser: u32,
    pub sign_algorithm: String,
    pub rbac: bool,
    pub permission_acc_token: bool,
    pub allow_skip_user: bool,
    pub allow_off_acc: bool,
    pub tenant_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseApiDetails {
    pub message: String,
    pub data: ApiDetails
}

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct LocalStorage {
//     pub username: Option<String>,
//     pub email: Option<String>,
//     pub token: Option<String>,
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct ResponseLogin {
//     pub email: String,
//     pub username: String,
//     pub token: String,
// }


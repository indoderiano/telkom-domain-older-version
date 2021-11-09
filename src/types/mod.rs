use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LocalStorage {
    pub username: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseLogin {
    pub email: String,
    pub username: String,
    pub token: String,
}

pub const LOCALSTORAGE_KEY: &str = "telkom-domain";


#[derive(Deserialize, Debug, Clone)]
pub struct ResponseMessage {
    pub message: String,
}



pub mod api;
pub mod settings;
pub mod roles;

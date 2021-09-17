use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Debug, Clone)]
pub struct ApiTitle {
    pub id: u8,
    pub name: String,
    pub api_type: String,
    pub identifier: String,
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


use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub age: u8
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseLogin {
    pub email: String,
    pub username: String,
    pub token: String,
}
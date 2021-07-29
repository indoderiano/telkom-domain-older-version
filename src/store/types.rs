use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub age: u8
}
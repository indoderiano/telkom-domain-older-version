use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl Role {
    pub fn new() -> Role {
        Role {
            id: String::from(""),
            name: String::from(""),
            description: String::from(""),
        }
    }
}
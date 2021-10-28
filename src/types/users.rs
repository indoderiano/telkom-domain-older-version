use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct GetUsersByIdResponseAppMetadata{}


#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct GetUsersByIdResponseUserMetadata{}

#[derive(Deserialize, Debug, Clone)]
pub struct UserTitle {
    pub user_id: String,
    pub email: String,
    pub email_verified: bool,
    pub username: String,
    pub phone_number: String,
    pub phone_verified: bool,
    pub created_at: String,
    pub updated_at: String,
    pub identities: Vec<UserIdentity>,
    pub app_metadata: GetUsersByIdResponseAppMetadata,
    pub user_metadata: GetUsersByIdResponseUserMetadata,
    pub picture: String,
    pub name: String,
    pub nickname: String,
    pub multifactor: Vec<String>,
    pub last_ip: String,
    pub last_login: String,
    pub logins_count: u32,
    pub blocked: bool,
    pub given_name: String,
    pub family_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseUsersList {
    pub message: String,
    pub data: Vec<UserTitle>,
}


#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct PostUsersByIdResponseAppMetadata{}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct PostUsersByIdResponseUserMetadata{}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct UserCreate {


    pub email: String,
    pub phone_number: String,
    pub user_metadata: PostUsersByIdResponseUserMetadata,
    pub blocked: bool,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub app_metadata : PostUsersByIdResponseAppMetadata,
    pub given_name: String,
    pub family_name: String,
    pub name: String,
    pub nickname: String,
    pub picture: String,
    pub user_id: String,
    pub connection: String,
    pub password: String,
    pub verify_email: String,
    pub username: String,
}

impl UserCreate {
    pub fn new() -> UserCreate {

        UserCreate {
            email: String::from(""),
            phone_number: String::from(""),
            user_metadata: PostUsersByIdResponseUserMetadata{},
            blocked: false,
            email_verified: false,
            phone_verified: false,
            app_metadata: PostUsersByIdResponseAppMetadata{},
            given_name: String::from(""),
            family_name: String::from(""),
            name: String::from(""),
            nickname: String::from(""),
            picture: String::from(""),
            user_id: String::from(""),
            connection: String::from(""),
            password: String::from(""),
            verify_email: String::from(""),
            username: String::from(""),
        }
    }
}



#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct UserDetails {
    pub user_id: String,
    pub email: String,
    pub email_verified: bool,
    pub username: String,
    pub phone_number: String,
    pub phone_verified: bool,
    pub created_at: String,
    pub updated_at: String,
    pub identities: Vec<UserIdentity>,
    pub app_metadata: GetUsersByIdResponseAppMetadata,
    pub user_metadata: GetUsersByIdResponseUserMetadata,
    pub picture: String,
    pub name: String,
    pub nickname: String,
    pub multifactor: Vec<String>,
    pub last_ip: String,
    pub last_login: String,
    pub logins_count: u32,
    pub blocked: bool,
    pub given_name: String,
    pub family_name: String,
}

impl UserDetails {
    pub fn new() -> UserDetails {
        UserDetails {
            user_id: String::from(""),
            email: String::from(""),
            email_verified: false,
            username: String::from(""),
            phone_number: String::from(""),
            phone_verified: false,
            created_at: String::from(""),
            updated_at: String::from(""),
            identities: vec![],
            app_metadata: GetUsersByIdResponseAppMetadata {},
            user_metadata: GetUsersByIdResponseUserMetadata {},
            picture: String::from(""),
            name: String::from(""),
            nickname: String::from(""),
            multifactor: vec![],
            last_ip: String::from(""),
            last_login: String::from(""),
            logins_count: 0,
            blocked: false,
            given_name: String::from(""),
            family_name: String::from(""),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct UserIdentity {
    pub connection: String,
    pub user_id: String,
    pub provider: String,
    pub is_social: bool,
}

// #[derive(Deserialize, Debug, Clone)]
// pub struct ResponseUserDetails {
//     pub data: UserDetails,
// }

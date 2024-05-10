use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize, PartialEq)]
#[sqlx(type_name = "role")]
pub enum Role {
    Admin,
    User,
}
impl From<String> for Role {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub role: Role,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct UserForCreate {
    pub id: i32,
    pub name: String,
    pub role: Role,
    pub password: String,
}
impl From<UserForCreate> for User {
    fn from(u: UserForCreate) -> Self {
        User {
            id: u.id,
            name: u.name,
            role: u.role,
            password: u.password,
        }
    }
}

#[derive(Deserialize)]
pub struct UserForLogin {
    pub id: i32,
    pub password: String,
}

use rocket::serde::Serialize;

use crate::services::types::user::User as ServiceUser;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub client_id: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Users {
    pub values: Vec<User>,
}

impl From<Vec<User>> for Users {
    fn from(values: Vec<User>) -> Self {
        return Self { values };
    }
}

impl From<ServiceUser> for User {
    fn from(user: ServiceUser) -> Self {
        return Self {
            id: user.id,
            client_id: user.client_id,
            is_admin: user.is_admin,
        };
    }
}

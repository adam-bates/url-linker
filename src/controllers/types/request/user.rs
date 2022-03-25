use rocket::serde::{json::Json, Deserialize};

use crate::services::types::user::{CreateUserRequest, UpdateUserRequest};

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub client_id: String,
    pub client_secret: String,
    pub is_admin: Option<bool>,
}

impl From<Json<CreateUser>> for CreateUser {
    fn from(json: Json<CreateUser>) -> Self {
        return json.0;
    }
}

impl Into<CreateUserRequest> for CreateUser {
    fn into(self) -> CreateUserRequest {
        return CreateUserRequest {
            client_id: self.client_id,
            client_secret: self.client_secret,
            is_admin: self.is_admin,
        };
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub is_admin: Option<bool>,
}

impl From<Json<UpdateUser>> for UpdateUser {
    fn from(json: Json<UpdateUser>) -> Self {
        return json.0;
    }
}

impl Into<UpdateUserRequest> for UpdateUser {
    fn into(self) -> UpdateUserRequest {
        return UpdateUserRequest {
            client_id: self.client_id,
            client_secret: self.client_secret,
            is_admin: self.is_admin,
        };
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserClientSecret {
    pub client_secret: String,
}

impl From<Json<UpdateUserClientSecret>> for UpdateUserClientSecret {
    fn from(json: Json<UpdateUserClientSecret>) -> Self {
        return json.0;
    }
}

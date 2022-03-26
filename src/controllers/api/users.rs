use rocket::{routes, serde::json::Json, Build, Rocket};

use crate::errors::user::UserError;
use crate::services::{
    types::{admin::Admin, user::User as ApiUser},
    user::UserService,
};

use super::super::types::{
    request::user::{CreateUser, UpdateUser, UpdateUserClientSecret},
    response::user::{User, Users},
};

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    return rocket.mount(
        "/api/v1/users",
        routes![
            get_self,
            update_self,
            create,
            get_all,
            get_by_id,
            update_by_id,
            delete_by_id
        ],
    );
}

#[get("/self")]
async fn get_self(user: ApiUser, user_service: Box<dyn UserService>) -> Result<User, UserError> {
    let user = user_service.get_by_id(user.id).await?;

    return Ok(User::from(user));
}

#[put("/self", data = "<user_client_secret>")]
async fn update_self(
    user: ApiUser,
    user_service: Box<dyn UserService>,
    user_client_secret: Json<UpdateUserClientSecret>,
) -> Result<User, UserError> {
    let body: UpdateUserClientSecret = user_client_secret.0;

    let user = user_service
        .update_self_client_secret(user, body.client_secret)
        .await?;

    return Ok(User::from(user));
}

#[post("/", data = "<user>")]
async fn create(
    _admin: Admin,
    user_service: Box<dyn UserService>,
    user: Json<CreateUser>,
) -> Result<User, UserError> {
    let user: CreateUser = user.0;

    let user = user_service.create(user.into()).await?;

    return Ok(User::from(user));
}

#[get("/?<client_id>")]
async fn get_all(
    _admin: Admin,
    user_service: Box<dyn UserService>,
    client_id: Option<String>,
) -> Result<Users, UserError> {
    let users = user_service.get_all(client_id).await?;

    return Ok(Users {
        values: users.into_iter().map(|user| User::from(user)).collect(),
    });
}

#[get("/<id>")]
async fn get_by_id(
    _admin: Admin,
    user_service: Box<dyn UserService>,
    id: i32,
) -> Result<User, UserError> {
    let user = user_service.get_by_id(id).await?;

    return Ok(User::from(user));
}

#[put("/<id>", data = "<user>")]
async fn update_by_id(
    _admin: Admin,
    user_service: Box<dyn UserService>,
    id: i32,
    user: Json<UpdateUser>,
) -> Result<User, UserError> {
    let user: UpdateUser = user.0;

    let user = user_service.update_by_id(id, user.into()).await?;

    return Ok(User::from(user));
}

#[delete("/<id>")]
async fn delete_by_id(
    _admin: Admin,
    user_service: Box<dyn UserService>,
    id: i32,
) -> Result<(), UserError> {
    user_service.delete_by_id(id).await?;

    return Ok(());
}

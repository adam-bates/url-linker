use rocket::{routes, serde::json::Json, Build, Rocket};

use crate::errors::user::UserError;
use crate::services::{
    types::{admin::Admin, user::User as AuthUser},
    user::UserService,
};

use super::super::types::{
    request::user::{CreateUser, UpdateUser, UpdateUserClientSecret},
    response::user::{User, Users},
};

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    return rocket.mount(
        "/api/users",
        routes![
            create,
            get_all,
            get_by_id,
            update_by_id,
            update_client_secret_by_id,
            delete_by_id
        ],
    );
}

#[post("/", data = "<user>")]
async fn create(
    user_service: Box<dyn UserService>,
    user: Json<CreateUser>,
    _admin: Admin,
) -> Result<User, UserError> {
    let user: CreateUser = user.0;

    let user = user_service.create(user.into()).await?;

    return Ok(User::from(user));
}

#[get("/")]
async fn get_all(user_service: Box<dyn UserService>, _admin: Admin) -> Result<Users, UserError> {
    let users = user_service.get_all().await?;

    return Ok(Users {
        values: users.into_iter().map(|user| User::from(user)).collect(),
    });
}

#[get("/<id>")]
async fn get_by_id(
    user_service: Box<dyn UserService>,
    id: i32,
    _admin: Admin,
) -> Result<User, UserError> {
    let user = user_service.get_by_id(id).await?;

    return Ok(User::from(user));
}

#[put("/<id>", data = "<user>")]
async fn update_by_id(
    user_service: Box<dyn UserService>,
    id: i32,
    user: Json<UpdateUser>,
    _admin: Admin,
) -> Result<User, UserError> {
    let user: UpdateUser = user.0;

    let user = user_service.update_by_id(id, user.into()).await?;

    return Ok(User::from(user));
}

#[put("/client_secret", data = "<user_client_secret>")]
async fn update_client_secret_by_id(
    user_service: Box<dyn UserService>,
    user_client_secret: Json<UpdateUserClientSecret>,
    user: AuthUser,
) -> Result<User, UserError> {
    let body: UpdateUserClientSecret = user_client_secret.0;

    let user = user_service
        .update_self_client_secret(user, body.client_secret)
        .await?;

    return Ok(User::from(user));
}

#[delete("/<id>")]
async fn delete_by_id(
    user_service: Box<dyn UserService>,
    id: i32,
    _admin: Admin,
) -> Result<(), UserError> {
    user_service.delete_by_id(id).await?;

    return Ok(());
}

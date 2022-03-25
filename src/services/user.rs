use crate::config::database::DbConnection;
use crate::errors::user::UserError;

use super::{
    password::PasswordService,
    types::user::{CreateUserRequest, UpdateUserRequest, User},
};

#[rocket::async_trait]
pub trait UserService: Send + Sync {
    async fn create(&self, user: CreateUserRequest) -> Result<User, UserError>;

    async fn get_all(&self) -> Result<Vec<User>, UserError>;

    async fn verify_and_get(
        &self,
        client_id: String,
        client_secret: String,
    ) -> Result<User, UserError>;

    async fn get_by_id(&self, id: i32) -> Result<User, UserError>;

    async fn update_by_id(&self, id: i32, user: UpdateUserRequest) -> Result<User, UserError>;

    async fn delete_by_id(&self, id: i32) -> Result<(), UserError>;
}

pub struct DbUserService {
    db: DbConnection,
    password_service: Box<dyn PasswordService>,
}

impl DbUserService {
    pub fn new(
        db: DbConnection,
        password_service: Box<dyn PasswordService>,
    ) -> Box<dyn UserService> {
        return Box::new(Self {
            db,
            password_service,
        });
    }
}

#[rocket::async_trait]
impl UserService for DbUserService {
    async fn create(&self, user: CreateUserRequest) -> Result<User, UserError> {
        let hash = self.password_service.generate_hash(&user.client_secret)?;

        let is_admin = user.is_admin.unwrap_or(false);

        let (id, client_id, is_admin) = self
            .db
            .run(move |connection| {
                for _ in connection
                    .query(
                        "SELECT client_id FROM users WHERE client_id = $1;",
                        &[&user.client_id],
                    )
                    .unwrap()
                {
                    return Err(UserError::ClientIdAlreadyExists);
                }

                let rows = connection
                    .execute(
                        "INSERT INTO users (client_id, client_secret, is_admin) VALUES ($1, $2, $3);",
                        &[&user.client_id, &hash, &is_admin],
                    )
                    .unwrap();

                if rows != 1 {
                    return Err(UserError::Unknown);
                }

                for row in connection
                    .query(
                        "SELECT id, client_id, is_admin FROM users WHERE client_id = $1;",
                        &[&user.client_id],
                    )
                    .unwrap()
                {
                    let id: i32 = row.get("id");

                    let value: &str = row.get("client_id");
                    let client_id = String::from(value);

                    let is_admin: bool = row.get("is_admin");

                    return Ok((id, client_id, is_admin));
                }

                return Err(UserError::Unknown);
            })
            .await?;

        return Ok(User {
            id,
            client_id,
            is_admin,
        });
    }

    async fn get_all(&self) -> Result<Vec<User>, UserError> {
        return self
            .db
            .run(move |connection| {
                let mut users = vec![];

                for row in connection
                    .query(
                        "SELECT id, client_id, client_secret, is_admin FROM users ORDER BY id ASC;",
                        &[],
                    )
                    .unwrap()
                {
                    let id: i32 = row.get("id");

                    let value: &str = row.get("client_id");
                    let client_id = String::from(value);

                    let is_admin: bool = row.get("is_admin");

                    users.push(User {
                        id,
                        client_id,
                        is_admin,
                    });
                }

                return Ok(users);
            })
            .await;
    }

    async fn verify_and_get(
        &self,
        client_id: String,
        client_secret: String,
    ) -> Result<User, UserError> {
        let (id, client_id, hash, is_admin) = self
            .db
            .run(move |connection| {
                for row in connection
                    .query(
                        "SELECT id, client_id, client_secret, is_admin FROM users WHERE client_id = $1;",
                        &[&client_id],
                    )
                    .unwrap()
                {
                    let id: i32 = row.get("id");

                    let value: &str = row.get("client_id");
                    let client_id = String::from(value);

                    let value: &str = row.get("client_secret");
                    let client_secret = String::from(value);

                    let is_admin: bool = row.get("is_admin");

                    return Ok((id, client_id, client_secret, is_admin));
                }

                return Err(UserError::NotFound);
            })
            .await?;

        let is_valid = self
            .password_service
            .verify_client_secret(&hash, &client_secret)?;

        if !is_valid {
            return Err(UserError::Invalid);
        }

        return Ok(User {
            id,
            client_id,
            is_admin,
        });
    }

    async fn get_by_id(&self, id: i32) -> Result<User, UserError> {
        let (id, client_id, is_admin) = self
            .db
            .run(move |connection| {
                for row in connection
                    .query(
                        "SELECT id, client_id, is_admin FROM users WHERE id = $1;",
                        &[&id],
                    )
                    .unwrap()
                {
                    let id: i32 = row.get("id");

                    let value: &str = row.get("client_id");
                    let client_id = String::from(value);

                    let is_admin: bool = row.get("is_admin");

                    return Ok((id, client_id, is_admin));
                }

                return Err(UserError::NotFound);
            })
            .await?;

        return Ok(User {
            id,
            client_id,
            is_admin,
        });
    }

    async fn update_by_id(&self, id: i32, user: UpdateUserRequest) -> Result<User, UserError> {
        let hash = match user.client_secret {
            Some(client_secret) => Some(self.password_service.generate_hash(&client_secret)?),
            None => None,
        };

        let (id, client_id, is_admin) = self
            .db
            .run(move |connection| {
                let mut found = false;

                for _ in connection
                    .query("SELECT id FROM users WHERE id = $1;", &[&id])
                    .unwrap()
                {
                    found = true;
                }

                if !found {
                    return Err(UserError::NotFound);
                }

                if let Some(client_id) = user.client_id {
                    for row in connection
                        .query(
                            "SELECT id, client_id FROM users WHERE client_id = $1;",
                            &[&client_id],
                        )
                        .unwrap()
                    {
                        let row_id: i32 = row.get("id");

                        if row_id != id {
                            return Err(UserError::ClientIdAlreadyExists);
                        }
                    }

                    let rows = connection
                        .execute(
                            "UPDATE users SET client_id = $1 WHERE id = $2;",
                            &[&client_id, &id],
                        )
                        .unwrap();

                    if rows != 1 {
                        return Err(UserError::Unknown);
                    }
                }

                if let Some(hash) = hash {
                    let rows = connection
                        .execute(
                            "UPDATE users SET client_secret = $1 WHERE id = $2;",
                            &[&hash, &id],
                        )
                        .unwrap();

                    if rows != 1 {
                        return Err(UserError::Unknown);
                    }
                }

                if let Some(is_admin) = user.is_admin {
                    let rows = connection
                        .execute(
                            "UPDATE users SET is_admin = $1 WHERE id = $2;",
                            &[&is_admin, &id],
                        )
                        .unwrap();

                    if rows != 1 {
                        return Err(UserError::Unknown);
                    }
                }

                for row in connection
                    .query(
                        "SELECT id, client_id, is_admin FROM users WHERE id = $1;",
                        &[&id],
                    )
                    .unwrap()
                {
                    let id: i32 = row.get("id");

                    let value: &str = row.get("client_id");
                    let client_id = String::from(value);

                    let is_admin: bool = row.get("is_admin");

                    return Ok((id, client_id, is_admin));
                }

                return Err(UserError::Unknown);
            })
            .await?;

        return Ok(User {
            id,
            client_id,
            is_admin,
        });
    }

    async fn delete_by_id(&self, id: i32) -> Result<(), UserError> {
        self.db
            .run(move |connection| {
                let mut found = false;

                for _ in connection
                    .query("SELECT id FROM users WHERE id = $1;", &[&id])
                    .unwrap()
                {
                    found = true;
                }

                if !found {
                    return Err(UserError::NotFound);
                }

                let rows = connection
                    .execute("DELETE FROM users WHERE id = $1;", &[&id])
                    .unwrap();

                if rows != 1 {
                    return Err(UserError::Unknown);
                }

                return Ok(());
            })
            .await?;

        return Ok(());
    }
}

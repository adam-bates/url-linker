use crate::config::database::DbConnection;
use crate::errors::user::UserError;

use super::types::user::{User, UserRequest};

#[rocket::async_trait]
pub trait UserService: Send + Sync {
    async fn create(&self, user: UserRequest) -> Result<User, UserError>;

    async fn get_all(&self) -> Result<Vec<User>, UserError>;

    async fn verify_and_get(&self, user: UserRequest) -> Result<User, UserError>;

    async fn get_by_id(&self, id: String) -> Result<User, UserError>;

    async fn update_by_id(&self, id: String, user: UserRequest) -> Result<User, UserError>;

    async fn delete_by_id(&self, id: String) -> Result<User, UserError>;
}

pub struct DbUserService {
    db: DbConnection,
}

impl DbUserService {
    pub fn new(db: DbConnection) -> Box<dyn UserService> {
        return Box::new(Self { db });
    }
}

#[rocket::async_trait]
impl UserService for DbUserService {
    async fn create(&self, user: UserRequest) -> Result<User, UserError> {
        todo!();
    }

    async fn get_all(&self) -> Result<Vec<User>, UserError> {
        todo!();
    }

    async fn verify_and_get(&self, user: UserRequest) -> Result<User, UserError> {
        todo!();
    }

    async fn get_by_id(&self, id: String) -> Result<User, UserError> {
        todo!();
    }

    async fn update_by_id(&self, id: String, user: UserRequest) -> Result<User, UserError> {
        todo!();
    }

    async fn delete_by_id(&self, id: String) -> Result<User, UserError> {
        todo!();
    }
}

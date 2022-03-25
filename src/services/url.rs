use crate::config::database::DbConnection;
use crate::errors::url::UrlError;

use super::types::{
    url::{CreateUrlRequest, UpdateUrlRequest, Url},
    user::User,
};

#[rocket::async_trait]
pub trait UrlService: Send + Sync {
    async fn create(&self, user: User, url: CreateUrlRequest) -> Result<Url, UrlError>;

    async fn get_all_by_user(&self, user: User) -> Result<Vec<Url>, UrlError>;

    async fn get_by_key_for_user(&self, user: User, key: String) -> Result<Url, UrlError>;

    async fn update_by_key_for_user(
        &self,
        user: User,
        key: String,
        url: UpdateUrlRequest,
    ) -> Result<Url, UrlError>;

    async fn delete_by_key_for_user(&self, user: User, key: String) -> Result<(), UrlError>;
}

pub struct DbUrlService {
    db: DbConnection,
}

impl DbUrlService {
    pub fn new(db: DbConnection) -> Box<dyn UrlService> {
        return Box::new(Self { db });
    }
}

#[rocket::async_trait]
impl UrlService for DbUrlService {
    async fn create(&self, user: User, url: CreateUrlRequest) -> Result<Url, UrlError> {
        let result = self
            .db
            .run(move |connection| {
                return connection.execute(
                    "INSERT INTO key_urls VALUES ($1, $2);",
                    &[&url.key, &url.url],
                );
            })
            .await;

        // result.map(|_| ()).map_err(|_| UrlError::KeyAlreadyExists);

        todo!();
    }

    async fn get_all_by_user(&self, user: User) -> Result<Vec<Url>, UrlError> {
        todo!();
    }

    async fn get_by_key_for_user(&self, user: User, key: String) -> Result<Url, UrlError> {
        // db.run(move |connection| {
        //     for row in connection
        //         .query("SELECT key, url FROM key_urls WHERE key = $1;", &[&key])
        //         .unwrap()
        //     {
        //         let url: &str = row.get("url");

        //         return String::from(url);
        //     }

        //     panic!("Not found!");
        // })
        // .await;

        todo!("{:?}", user)
    }

    async fn update_by_key_for_user(
        &self,
        user: User,
        key: String,
        url: UpdateUrlRequest,
    ) -> Result<Url, UrlError> {
        todo!()
    }

    async fn delete_by_key_for_user(&self, user: User, key: String) -> Result<(), UrlError> {
        todo!()
    }
}

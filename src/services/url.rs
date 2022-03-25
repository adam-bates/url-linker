use crate::config::database::DbConnection;
use crate::errors::url::UrlError;

use super::types::{
    url::{CreateUrlRequest, UpdateUrlRequest, Url},
    user::User,
};

#[rocket::async_trait]
pub trait UrlService: Send + Sync {
    async fn create(&self, user: User, url: CreateUrlRequest) -> Result<Url, UrlError>;

    async fn get_all(&self) -> Result<Vec<Url>, UrlError>;

    async fn get_all_by_user(&self, user: User) -> Result<Vec<Url>, UrlError>;

    async fn get_by_key(&self, key: String) -> Result<Url, UrlError>;

    async fn get_by_key_for_user(&self, user: User, key: String) -> Result<Url, UrlError>;

    async fn update_by_key(&self, key: String, url: UpdateUrlRequest) -> Result<Url, UrlError>;

    async fn update_by_key_for_user(
        &self,
        user: User,
        key: String,
        url: UpdateUrlRequest,
    ) -> Result<Url, UrlError>;

    async fn delete_by_key(&self, key: String) -> Result<(), UrlError>;

    async fn delete_by_key_for_user(&self, user: User, key: String) -> Result<(), UrlError>;

    async fn delete_by_user_id(&self, user_id: i32) -> Result<(), UrlError>;
}

pub struct DbUrlService {
    db: DbConnection,
}

impl DbUrlService {
    pub fn new(db: DbConnection) -> Box<dyn UrlService> {
        return Box::new(Self { db });
    }
}

fn validate_key(key: &str) -> Result<(), UrlError> {
    const MIN: usize = 1;
    const MAX: usize = 128;

    let length = key.len();

    if length < MIN {
        return Err(UrlError::KeyTooShort { min: MIN });
    }

    if length > MAX {
        return Err(UrlError::KeyTooLong { max: MAX });
    }

    return Ok(());
}

fn validate_url(url: &str) -> Result<(), UrlError> {
    let url = url::Url::parse(url).map_err(|e| UrlError::UrlParseError(e.to_string()))?;

    if url.domain().is_none() {
        return Err(UrlError::UrlInvalid);
    }

    let scheme = url.scheme();

    if scheme != "http" && scheme != "https" {
        return Err(UrlError::UrlInvalid);
    }

    return Ok(());
}

#[rocket::async_trait]
impl UrlService for DbUrlService {
    async fn create(&self, user: User, url: CreateUrlRequest) -> Result<Url, UrlError> {
        validate_key(&url.key)?;
        validate_url(&url.url)?;

        let key = url.key.to_ascii_lowercase();

        let (key, url, user_id) = self
            .db
            .run(move |connection| {
                for _ in connection
                    .query("SELECT key FROM key_urls WHERE key = $1;", &[&key])
                    .unwrap()
                {
                    return Err(UrlError::KeyAlreadyExists);
                }

                let rows = connection
                    .execute(
                        "INSERT INTO key_urls (key, url, user_id) VALUES ($1, $2, $3);",
                        &[&key, &url.url, &user.id],
                    )
                    .unwrap();

                if rows != 1 {
                    return Err(UrlError::Unknown);
                }

                for row in connection
                    .query(
                        "SELECT key, url, user_id FROM key_urls WHERE key = $1;",
                        &[&key],
                    )
                    .unwrap()
                {
                    let value: &str = row.get("key");
                    let key = String::from(value);

                    let value: &str = row.get("url");
                    let url = String::from(value);

                    let user_id: i32 = row.get("user_id");

                    return Ok((key, url, user_id));
                }

                return Err(UrlError::Unknown);
            })
            .await?;

        return Ok(Url { key, url, user_id });
    }

    async fn get_all(&self) -> Result<Vec<Url>, UrlError> {
        return self
            .db
            .run(move |connection| {
                let mut urls = vec![];

                for row in connection
                    .query(
                        "SELECT key, url, user_id FROM key_urls ORDER BY key ASC;",
                        &[],
                    )
                    .unwrap()
                {
                    let value: &str = row.get("key");
                    let key = String::from(value);

                    let value: &str = row.get("url");
                    let url = String::from(value);

                    let user_id: i32 = row.get("user_id");

                    urls.push(Url { key, url, user_id });
                }

                return Ok(urls);
            })
            .await;
    }

    async fn get_all_by_user(&self, user: User) -> Result<Vec<Url>, UrlError> {
        return self
            .db
            .run(move |connection| {
                let mut urls = vec![];

                for row in connection
                    .query(
                        "SELECT key, url, user_id FROM key_urls WHERE user_id = $1 ORDER BY key ASC;",
                        &[&user.id],
                    )
                    .unwrap()
                {
                    let value: &str = row.get("key");
                    let key = String::from(value);

                    let value: &str = row.get("url");
                    let url = String::from(value);

                    let user_id: i32 = row.get("user_id");

                    urls.push(Url { key, url, user_id });
                }

                return Ok(urls);
            })
            .await;
    }

    async fn get_by_key(&self, key: String) -> Result<Url, UrlError> {
        let key = key.to_ascii_lowercase();

        let (key, url, user_id) = self
            .db
            .run(move |connection| {
                for row in connection
                    .query(
                        "SELECT key, url, user_id FROM key_urls WHERE key = $1;",
                        &[&key],
                    )
                    .unwrap()
                {
                    let value: &str = row.get("key");
                    let key = String::from(value);

                    let value: &str = row.get("url");
                    let url = String::from(value);

                    let user_id: i32 = row.get("user_id");

                    return Ok((key, url, user_id));
                }

                return Err(UrlError::NotFound);
            })
            .await?;

        return Ok(Url { key, url, user_id });
    }

    async fn get_by_key_for_user(&self, user: User, key: String) -> Result<Url, UrlError> {
        let key = key.to_ascii_lowercase();

        let (key, url, user_id) = self
            .db
            .run(move |connection| {
                for row in connection
                    .query(
                        "SELECT key, url, user_id FROM key_urls WHERE key = $1 AND user_id = $2;",
                        &[&key, &user.id],
                    )
                    .unwrap()
                {
                    let value: &str = row.get("key");
                    let key = String::from(value);

                    let value: &str = row.get("url");
                    let url = String::from(value);

                    let user_id: i32 = row.get("user_id");

                    return Ok((key, url, user_id));
                }

                return Err(UrlError::NotFound);
            })
            .await?;

        return Ok(Url { key, url, user_id });
    }

    async fn update_by_key(&self, key: String, url: UpdateUrlRequest) -> Result<Url, UrlError> {
        let mut key = key.to_ascii_lowercase();

        if let Some(key) = &url.key {
            validate_key(key)?;
        }

        if let Some(url) = &url.url {
            validate_url(url)?;
        }

        let (key, url, user_id) = self
            .db
            .run(move |connection| {
                let mut found = false;

                for _ in connection
                    .query("SELECT key FROM key_urls WHERE key = $1;", &[&key])
                    .unwrap()
                {
                    found = true;
                }

                if !found {
                    return Err(UrlError::NotFound);
                }

                if let Some(url_key) = url.key {
                    let url_key = url_key.to_ascii_lowercase();

                    if url_key != key {
                        for _ in connection
                            .query("SELECT key FROM key_urls WHERE key = $1;", &[&url_key])
                            .unwrap()
                        {
                            return Err(UrlError::KeyAlreadyExists);
                        }
                    }

                    let rows = connection
                        .execute(
                            "UPDATE key_urls SET key = $1 WHERE key = $2;",
                            &[&url_key, &key],
                        )
                        .unwrap();

                    if rows != 1 {
                        return Err(UrlError::Unknown);
                    }

                    key = url_key;
                }

                if let Some(url) = url.url {
                    let rows = connection
                        .execute(
                            "UPDATE key_urls SET url = $1 WHERE key = $2;",
                            &[&url, &key],
                        )
                        .unwrap();

                    if rows != 1 {
                        return Err(UrlError::Unknown);
                    }
                }

                for row in connection
                    .query(
                        "SELECT key, url, user_id FROM key_urls WHERE key = $1;",
                        &[&key],
                    )
                    .unwrap()
                {
                    let value: &str = row.get("key");
                    let key = String::from(value);

                    let value: &str = row.get("url");
                    let url = String::from(value);

                    let user_id: i32 = row.get("user_id");

                    return Ok((key, url, user_id));
                }

                return Err(UrlError::Unknown);
            })
            .await?;

        return Ok(Url { key, url, user_id });
    }

    async fn update_by_key_for_user(
        &self,
        user: User,
        key: String,
        url: UpdateUrlRequest,
    ) -> Result<Url, UrlError> {
        let mut key = key.to_ascii_lowercase();

        if let Some(key) = &url.key {
            validate_key(key)?;
        }

        if let Some(url) = &url.url {
            validate_url(url)?;
        }

        let (key, url, user_id) = self
            .db
            .run(move |connection| {
                let mut found = false;

                for _ in connection
                    .query(
                        "SELECT key FROM key_urls WHERE key = $1 AND user_id = $2;",
                        &[&key, &user.id],
                    )
                    .unwrap()
                {
                    found = true;
                }

                if !found {
                    return Err(UrlError::NotFound);
                }

                if let Some(url_key) = url.key {
                    let url_key = url_key.to_ascii_lowercase();

                    if url_key != key {
                        for _ in connection
                            .query("SELECT key FROM key_urls WHERE key = $1;", &[&url_key])
                            .unwrap()
                        {
                            return Err(UrlError::KeyAlreadyExists);
                        }
                    }

                    let rows = connection
                        .execute(
                            "UPDATE key_urls SET key = $1 WHERE key = $2;",
                            &[&url_key, &key],
                        )
                        .unwrap();

                    if rows != 1 {
                        return Err(UrlError::Unknown);
                    }

                    key = url_key;
                }

                if let Some(url) = url.url {
                    let rows = connection
                        .execute(
                            "UPDATE key_urls SET url = $1 WHERE key = $2;",
                            &[&url, &key],
                        )
                        .unwrap();

                    if rows != 1 {
                        return Err(UrlError::Unknown);
                    }
                }

                for row in connection
                    .query(
                        "SELECT key, url, user_id FROM key_urls WHERE key = $1;",
                        &[&key],
                    )
                    .unwrap()
                {
                    let value: &str = row.get("key");
                    let key = String::from(value);

                    let value: &str = row.get("url");
                    let url = String::from(value);

                    let user_id: i32 = row.get("user_id");

                    return Ok((key, url, user_id));
                }

                return Err(UrlError::Unknown);
            })
            .await?;

        return Ok(Url { key, url, user_id });
    }

    async fn delete_by_key(&self, key: String) -> Result<(), UrlError> {
        let key = key.to_ascii_lowercase();

        self.db
            .run(move |connection| {
                let mut found = false;

                for _ in connection
                    .query("SELECT key FROM key_urls WHERE key = $1;", &[&key])
                    .unwrap()
                {
                    found = true;
                }

                if !found {
                    return Err(UrlError::NotFound);
                }

                let rows = connection
                    .execute("DELETE FROM key_urls WHERE key = $1;", &[&key])
                    .unwrap();

                if rows != 1 {
                    return Err(UrlError::Unknown);
                }

                return Ok(());
            })
            .await?;

        return Ok(());
    }

    async fn delete_by_key_for_user(&self, user: User, key: String) -> Result<(), UrlError> {
        let key = key.to_ascii_lowercase();

        self.db
            .run(move |connection| {
                let mut found = false;

                for _ in connection
                    .query(
                        "SELECT key FROM key_urls WHERE key = $1 AND user_id = $2;",
                        &[&key, &user.id],
                    )
                    .unwrap()
                {
                    found = true;
                }

                if !found {
                    return Err(UrlError::NotFound);
                }

                let rows = connection
                    .execute(
                        "DELETE FROM key_urls WHERE key = $1 AND user_id = $2;",
                        &[&key, &user.id],
                    )
                    .unwrap();

                if rows != 1 {
                    return Err(UrlError::Unknown);
                }

                return Ok(());
            })
            .await?;

        return Ok(());
    }

    async fn delete_by_user_id(&self, user_id: i32) -> Result<(), UrlError> {
        self.db
            .run(move |connection| {
                let _ = connection
                    .execute("DELETE FROM key_urls WHERE user_id = $1;", &[&user_id])
                    .unwrap();

                return Ok(());
            })
            .await?;

        return Ok(());
    }
}

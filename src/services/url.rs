use rocket::request::{FromRequest, Outcome, Request};

use super::{errors::UrlError, types::Url, DbConnection};

pub struct UrlService {
    db: DbConnection,
}

impl UrlService {
    pub async fn create(&self, url: Url) -> Result<(), UrlError> {
        let result = self
            .db
            .run(move |connection| {
                return connection.execute(
                    "INSERT INTO key_urls VALUES ($1, $2);",
                    &[&url.key, &url.url],
                );
            })
            .await;

        return result.map(|_| ()).map_err(|_| UrlError::KeyAlreadyExists);
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UrlService {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        return match req.guard::<DbConnection>().await {
            Outcome::Success(db) => Outcome::Success(UrlService { db }),
            Outcome::Failure(e) => Outcome::Failure(e),
            Outcome::Forward(e) => Outcome::Forward(e),
        };
    }
}

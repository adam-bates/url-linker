use uuid::Uuid;

use crate::errors::user::UserError;

pub trait PasswordService: Send + Sync {
    fn generate_hash(&self, client_secret: &str) -> Result<String, UserError>;

    fn verify_client_secret(&self, hash: &str, client_secret: &str) -> Result<bool, UserError>;
}

pub type Argon2Config = argon2::Config<'static>;
pub type Argon2ConfigRef = std::sync::Arc<Argon2Config>;

pub struct Argon2PasswordService {
    config: Argon2ConfigRef,
}

impl Argon2PasswordService {
    pub fn new(config: Argon2ConfigRef) -> Box<dyn PasswordService> {
        return Box::new(Argon2PasswordService { config });
    }
}

impl PasswordService for Argon2PasswordService {
    fn generate_hash(&self, client_secret: &str) -> Result<String, UserError> {
        let salt = Uuid::new_v4();

        return argon2::hash_encoded(client_secret.as_bytes(), salt.as_bytes(), &self.config)
            .map_err(|e| UserError::HashError(e.to_string()));
    }

    fn verify_client_secret(&self, hash: &str, client_secret: &str) -> Result<bool, UserError> {
        return argon2::verify_encoded_ext(hash, client_secret.as_bytes(), self.config.secret, &[])
            .map_err(|e| UserError::HashError(e.to_string()));
    }
}

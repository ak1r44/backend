use crate::{models::roles::Role, utils::crypto::CryptoUtil};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub role: Role,
}

impl User {
    pub fn new(id: Uuid, username: String, password: String, role: Role) -> Self {
        User {
            id,
            username,
            password,
            role,
        }
    }

    /// Validate the password.
    pub fn validate(&self, password: &str) -> bool {
        let crypto = CryptoUtil::new();

        match crypto {
            Ok(c) => c.verify(&self.password, password).unwrap_or(false),
            Err(_) => false,
        }
    }
}

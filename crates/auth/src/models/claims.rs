use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    permissions::{Permission, PermissionSet},
    roles::Role,
};

const TOKEN_DURATION: Duration = Duration::days(1);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub uid: Uuid,
    pub iat: i64,
    pub role: Role,
    pub permissions: PermissionSet,
    pub exp: i64,
}

impl Claims {
    pub fn new(uid: Uuid, role: Role) -> Self {
        Claims {
            uid,
            iat: Utc::now().timestamp(),
            role: role.clone(),
            permissions: role.permissions(),
            exp: (Utc::now() + TOKEN_DURATION).timestamp(),
        }
    }

    pub fn encode(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            self,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
        )
    }

    pub fn decode(token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map(|data| data.claims)
    }

    pub fn validate(&self) -> bool {
        Utc::now().timestamp() < self.exp
    }

    pub fn has(&self, permission: Permission) -> bool {
        self.permissions.contains(permission)
    }
}
